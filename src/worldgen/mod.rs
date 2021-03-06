extern crate rand;
extern crate tcod_sys;

use std;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::{Index, Range};

use tcod::noise::{Noise, NoiseType};
use tcod::random;

pub mod terrain;
use self::rand::Rng;
use self::rand::SeedableRng;
use self::terrain::*;

use life::{Living, MissionResult, Order};

use physics::PhysicsActor;

use time::{Calendar, Clock, Time};

use utils::{Point2D, Point3D, strict_adjacent};

/// Returns the Some() of the restricted version of a Tile if it can be restricted, if not, returns None.
fn restricted_from_tile(tile: Tile) -> Option<RestrictedTile> {
    match tile {
        Tile::Stone(a, b) => Some(RestrictedTile::Stone(a, b)),
        Tile::Vegetation(a, b, c) => {
            Some(RestrictedTile::Vegetation(a, b, c))
        }
        _ => None,
    }
}

/// A 1x1 cross section of the layered world, including a ref to the
/// biome it is part of.
#[derive(Clone)]
pub struct Unit {
    pub biome: Option<Biome>,
    pub tiles: RefCell<Vec<Tile>>,
}

/// A mapping of a unique string (based on the type of thing animated)
/// to its FrameAssoc.
pub type Frames = HashMap<String, Vec<usize>>;

/// A type alias for the world's map.
pub type WorldMap = Vec<Vec<Unit>>;

/// A type alias for the biome map.
pub type BiomeMap = HashMap<String, RefCell<Vec<(usize, usize)>>>;

/// Keeps track of the state of they physical world, including:
/// * the heightmap
/// * the vegitation and stone noise
/// * the animation state
/// * the map size and unit map
pub struct World {
    heightmap: *mut tcod_sys::TCOD_heightmap_t,
    stone_vein_noise: Noise,
    seed: u32,
    pub map_size: Point2D,
    pub frames: Frames,
    pub biome_map: BiomeMap,
    pub map: WorldMap,
    pub life: Vec<RefCell<Box<Living>>>,
}

impl Index<usize> for World {
    type Output = Vec<Unit>;
    fn index(&self, location: usize) -> &Self::Output {
        &self.map[location]
    }
}

impl Index<Range<usize>> for World {
    type Output = [Vec<Unit>];
    fn index(&self, location: Range<usize>) -> &Self::Output {
        &self.map[location]
    }
}

const THRESHOLD: f32 = 0.3;
const SEA_LEVEL: Cell<f32> = Cell::new(13.0);
const WATER_LEVEL: f32 = 7.0;
const VEG_THRESHOLD: f32 = 200.0;
const RAMP_THRESHOLD: f32 = 0.015;
const ANIMAL_COUNT: usize = 50;

impl World {
    /// Generates a new hightmap-based world map of the specified
    /// size. The generation order goes roughly thus:
    /// * generate noise for vegitation
    /// * create a heightmap
    /// * generate low-level noise for the heightmap
    /// * add the noise (through FBM) to the heightmap
    /// * run erosion simulation on heightmap
    /// * add randomly sized hills
    /// * dig randomly sized hills
    pub fn new(size: Point2D, seed: u32) -> World {
        println!("Generating world from seed {}", seed);
        let rng = random::Rng::new_with_seed(random::Algo::MT, seed);
        SEA_LEVEL.set(rng.get_float(14.0, 17.0));

        // Vegetation
        let mut world: World = World {
            map_size: size,
            heightmap: Self::generate_heightmap(size, seed),
            map: vec![],
            biome_map: ["s", "j", "f", "d", "p", "b", "w"]
                .iter()
                .cloned()
                .map(|x| (x.to_string(), RefCell::new(vec![])))
                .collect(),
            stone_vein_noise: Noise::init_with_dimensions(3)
                .lacunarity(0.43)
                .hurst(-0.9)
                .noise_type(NoiseType::Simplex)
                .random(rng)
                .init(),
            life: vec![],
            seed: seed,
            frames: [("Water".to_string(),
                      vec![16, 32, 33, 34, 35, 36])]
                    .iter()
                    .cloned()
                    .map(|(x, y)| (x, y))
                    .collect(),
        };
        world.map = Self::map_from(size, &world);
        world
    }

    pub fn kill(&mut self, i: usize) {
        let l = self.life.remove(i).into_inner();
        let pos = l.current_pos();
        self.map[pos.0][pos.1]
            .tiles
            .borrow_mut()
            [pos.2] = Tile::Item(Item::Food(Food::Meat(l.species()
                                                        .species)))
    }

    pub fn create_life_by_biome(pnt: Point3D,
                                biome: Biome)
        -> Option<Box<Living>> {
        use life::animal::*;
        let mut trng = rand::thread_rng();
        let species = match biome.biome_type {
            BiomeType::Water => {
                if biome.temperature_night_f < 50.0 {
                    Species::Herbivore(Herbivore::Whale)
                } else if biome.temperature_day_f > 80.0 {
                    Species::Carnivore(Carnivore::Shark)
                } else {
                    Species::Herbivore(Herbivore::Fish)
                }
            }
            BiomeType::Desert | BiomeType::Beach => {
                return None;
            }
            BiomeType::Forest => {
                *trng.choose(
                    &[if biome.temperature_day_f > 70.0 {
                          Species::Carnivore(Carnivore::Dog)
                      } else if biome.temperature_day_f < 60.0 {
                          Species::Carnivore(Carnivore::Wolf)
                      } else {
                          Species::Carnivore(Carnivore::Cat)
                      },
                      Species::Herbivore(Herbivore::Rabbit),
                      Species::Carnivore(Carnivore::Wolf)],
                )
                     .unwrap()
            }
            BiomeType::Jungle => {
                *trng.choose(
                    &[
                        Species::Herbivore(Herbivore::Hippo),
                        Species::Carnivore(Carnivore::Alligator),
                        Species::Carnivore(Carnivore::Wolf),
                        Species::Herbivore(Herbivore::Fish),
                    ],
                ).unwrap()
            }
            BiomeType::Pasture => {
                *trng.choose(
                    &[
                        Species::Herbivore(Herbivore::Sheep),
                        Species::Herbivore(Herbivore::Cow),
                        Species::Herbivore(Herbivore::Rabbit),
                        Species::Herbivore(Herbivore::Armadillo),
                        Species::Carnivore(Carnivore::Dog),
                        Species::Carnivore(Carnivore::Cat),
                    ],
                ).unwrap()
            }
            BiomeType::Swamp => {
                *trng.choose(
                    &[
                        Species::Carnivore(Carnivore::Alligator),
                        Species::Carnivore(Carnivore::Shark),
                        Species::Herbivore(Herbivore::Fish),
                        Species::Herbivore(Herbivore::Hippo),
                        Species::Herbivore(Herbivore::Armadillo),
                    ],
                ).unwrap()
            }
        };

        Some(Animal::new(pnt, species))
    }


    /// Creates a vector of animals based on biome and height.
    pub fn generate_life(&mut self) {
        let mut rng = rand::IsaacRng::from_seed(&[self.seed]);
        for _ in 0..ANIMAL_COUNT {
            let bks = self.biome_map
                          .iter()
                          .map(|(k, _)| k.clone())
                          .collect::<Vec<_>>();
            let chosen_biome = rng.choose(&bks).unwrap();
            let bps = &self.biome_map[chosen_biome];
            if let Some(point) = rng.choose(&bps.borrow()) {
                let unit = self.get(*point).unwrap();
                let z = self.location_z(*point);
                let ut = unit.tiles.borrow();
                let tile = ut.get(z);
                if tile.is_some() && !tile.unwrap().solid() {
                    let p3d = (point.0, point.1, z);
                    let animal =
                        World::create_life_by_biome(p3d,
                                                    unit.biome
                                                        .unwrap());
                    if let Some(animal) = animal {
                        self.life.push(RefCell::new(animal));
                    }
                }
            }
        }
    }

    unsafe fn get_slope(&self, x: usize, y: usize) -> f32 {
        tcod_sys::TCOD_heightmap_get_slope(self.heightmap,
                                           x as i32,
                                           y as i32)
    }

    /// Step 1 of map generation:
    /// Generate bedrock and mountains/hills from terrain info
    fn rock_from_terrain(ws: &World,
                         (sw, sh): Point2D,
                         world: WorldMap)
        -> WorldMap {
        (0..sh)
            .map(|y| {
                (0..sw)
                    .map(|x| {
                        let height = unsafe { ws.get_height(x, y) } as usize;
                        Unit {
                            biome: None,
                            tiles: RefCell::new(
                                (0..std::cmp::max(height, 5))
                                    .map(|h| ws.rock_type((x, y), h as isize))
                                    .collect(),
                            ),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    /// Step 2 of map generation:
    /// Replace low rock with water of a similar depth.
    /// The sea level is raised when inland to allow for rivers and pools.
    fn water_from_low(world: WorldMap) -> WorldMap {
        world.iter()
             .enumerate()
             .map(|(y, row)| {
            row.iter()
               .enumerate()
               .map(|(x, unit)| {
                let landlocked =
                    !strict_adjacent((x, y))
                        .iter()
                        .map(|&(x, y)| {
                            let row = get!(world.get(y));
                            let unit = get!(row.get(x));
                            Some(unit.tiles.borrow().len() >
                                     SEA_LEVEL.get() as usize)
                        })
                        .any(
                            |thing| thing.is_some() && thing.unwrap(),
                        );

                let ut = unit.tiles.borrow();
                let unit_height = ut.len();
                let water_unit = Unit {
                    biome: Some(WATER_BIOME),
                    tiles: RefCell::new(
                        (0..unit_height)
                            .map(|depth| if depth <=
                                unit_height.checked_sub(
                                    WATER_LEVEL as usize,
                                )
                                           .unwrap_or(0)
                            {
                                ut[depth].clone()
                            } else {
                                Tile::Water(World::purity(),
                                            State::Liquid,
                                            unit_height as i32 -
                                                depth as i32)
                            })
                            .collect(),
                    ),
                };
                if (unit_height < SEA_LEVEL.get() as usize) ||
                    (landlocked &&
                         unit_height < SEA_LEVEL.get() as usize + 2)
                {
                    water_unit
                } else {
                    unit.clone()
                }
            })
               .collect::<Vec<_>>()
        })
             .collect::<Vec<_>>()
    }

    /// Step 3 of map generation:
    /// Generate biomes (temp, percipitation) from altitude and a noise
    fn biomes_from_height_and_noise(world_map: WorldMap,
                                    world: &World)
        -> WorldMap {
        let bnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.43)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT,
                                               world.seed))
            .init();
        world_map.iter()
                 .enumerate()
                 .map(|(y, row)| {
            row.iter()
               .enumerate()
               .map(|(x, unit)| {
                let (sh, n) =
                    strict_adjacent((x, y))
                        .iter()
                        .map(|&(x, y)| {
                            let row = get!(world_map.get(y));
                            let unit = get!(row.get(x));
                            Some(unit.tiles.borrow().len())
                        })
                        .filter_map(|x| x)
                        .fold((0, 0), |(s, n), x| (s + x, n + 1));
                let noise = bnoise.get_fbm([x as f32, y as f32], 6) *
                    100.0;
                let biome = World::biome_from_noise(noise as i32,
                                                    (sh as i32 / n) as
                                                        f32);
                let bn = &biome.biome_type.stringified();
                if let Some(bml) = world.biome_map.get(bn) {
                    bml.borrow_mut().push((x, y));
                }
                Unit {
                    tiles: unit.tiles.clone(),
                    biome: Some(unit.biome.unwrap_or(biome)),
                }
            })
               .collect::<Vec<_>>()
        })
                 .collect::<Vec<_>>()
    }

    /// Step 4 of map generation:
    /// Generate vegitation based on what survives where in the biomes, and height.
    fn vegitation_from_biomes(world: WorldMap,
                              seed: u32)
        -> WorldMap {
        let vnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT,
                                               seed))
            .init();
        world.iter()
             .enumerate()
             .map(|(y, row)| {
            row.iter()
               .enumerate()
               .map(|(x, unit)| if let Some(biome) = unit.biome {
                if biome.biome_type != BiomeType::Water {
                    let mut tiles = unit.tiles.clone().into_inner();
                    tiles.push(World::get_vegetation(&vnoise,
                                                     (x, y),
                                                     biome));
                    Unit {
                        biome: unit.biome,
                        tiles: RefCell::new(tiles),
                    }
                } else {
                    // Seaweed from bottom to not quite surface. For
                    // fish and wales to eat.
                    unit.clone()
                }
            } else {
                unreachable!()
            })
               .collect::<Vec<_>>()
        })
             .collect::<Vec<_>>()
    }

    /// Step 5 of map generation:
    /// Generate the soil (and snow) based on plant and biome.
    fn add_soil(world: WorldMap, seed: u32) -> WorldMap {
        fn get_op<'a>(p: Point2D,
                      world: &'a WorldMap)
            -> Option<&'a Unit> {
            let row = get!(world.get(p.1));
            let unit = get!(row.get(p.0));
            Some(unit)
        }

        world
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, unit)| {
                        let mut t = unit.tiles.clone().into_inner();
                        for h in (SEA_LEVEL.get() as i32)..(SEA_LEVEL.get() as i32 + 4) {
                            let adj = strict_adjacent((x, y))
                                .iter()
                                .filter_map(|p| get_op(*p, &world).cloned())
                                .collect::<Vec<_>>();
                            if (h as usize) < t.len() {
                                t[h as usize] = Tile::Stone(
                                    StoneTypes::Soil(World::soil_choice(h, adj, seed)),
                                    State::Solid,
                                );
                            } else {
                                break;
                            }
                        }
                        Unit {
                            biome: unit.biome,
                            tiles: RefCell::new(t),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    /// Generates a new unit map for World from the given (incomplete) World.
    /// The steps go as follows:
    /// * Generate bedrock and mountains/hills from terrain info
    /// * Replace low rock with water (for sea, pools)
    /// * Generate biomes (temp, percipitation) from altitude and a noise
    /// * Generate vegitation based on what survives where in the biomes
    /// * Generate the soil (and snow) based on plant and biome.
    fn map_from(size: Point2D, ws: &World) -> WorldMap {
        let rock_from_terrain = World::rock_from_terrain;
        let water_from_low = World::water_from_low;
        let biomes_from_height_and_noise =
            World::biomes_from_height_and_noise;
        let vegitation_from_biomes = World::vegitation_from_biomes;
        let add_soil = World::add_soil;
        pipe!(
            vec![]
                => {|i| rock_from_terrain(ws, size, i)}
            => water_from_low
            => { |x| biomes_from_height_and_noise(x, ws) }
            => { |x| vegitation_from_biomes(x, ws.seed) }
            => { |x| add_soil(x, ws.seed) }
        )
    }


    pub fn life_at_point(
        &self,
        x: usize,
        y: usize)
        -> Option<(usize, &RefCell<Box<Living>>)> {
        self.life
            .iter()
            .enumerate()
            .find(|&(_i, e)| {
            let op = e.borrow().current_pos();
            (op.0, op.1) == (x, y)
        })
    }

    /// A general method for dealing with generating random hills of a limited size, position, and height.
    fn random_hill_operation<F>(
        heightmap: *mut tcod_sys::TCOD_heightmap_t,
        (hmw, hmh): Point2D,
        num_hills: i32,
        base_radius: f32,
        radius: f32,
        height: f32,
        rndn: tcod_sys::TCOD_random_t,
        operation: &F,
    ) where
        F: Fn(*mut tcod_sys::TCOD_heightmap_t, f32, f32, f32, f32),
    {
        unsafe {
            for _ in 0..num_hills {
                let radius =
                    tcod_sys::TCOD_random_get_float(rndn,
                                                    base_radius *
                                                        (1.0 -
                                                             radius),
                                                    base_radius *
                                                        (1.0 +
                                                             radius));
                let xh = tcod_sys::TCOD_random_get_int(rndn,
                                                       0,
                                                       hmw as i32 -
                                                           1);
                let yh = tcod_sys::TCOD_random_get_int(rndn,
                                                       0,
                                                       hmh as i32 -
                                                           1);
                operation(heightmap,
                          xh as f32,
                          yh as f32,
                          radius,
                          height);
            }
        }
    }

    /// Extrudes random hills.
    fn add_random_hills(hm: *mut tcod_sys::TCOD_heightmap_t,
                        sz: Point2D,
                        nh: i32,
                        br: f32,
                        r: f32,
                        h: f32,
                        n: tcod_sys::TCOD_random_t) {
        Self::random_hill_operation(hm,
                                    sz,
                                    nh,
                                    br,
                                    r,
                                    h,
                                    n,
                                    &|a, b, c, d, e| unsafe { tcod_sys::TCOD_heightmap_add_hill(a, b, c, d, e) });
    }

    /// Digs random hills.
    fn dig_random_hills(hm: *mut tcod_sys::TCOD_heightmap_t,
                        sz: Point2D,
                        nh: i32,
                        br: f32,
                        r: f32,
                        h: f32,
                        n: tcod_sys::TCOD_random_t) {
        Self::random_hill_operation(hm,
                                    sz,
                                    nh,
                                    br,
                                    r,
                                    h,
                                    n,
                                    &|a, b, c, d, e| unsafe { tcod_sys::TCOD_heightmap_dig_hill(a, b, c, d, e) });
    }

    /// Generates a new heightmap: this is all unsafe code, and uses the low-level FFI to libtcod 5.1.
    fn generate_heightmap((sx, sy): Point2D,
                          seed: u32)
        -> *mut tcod_sys::TCOD_heightmap_t {
        let heightmap = unsafe {
            tcod_sys::TCOD_heightmap_new(sx as i32, sy as i32)
        };
        unsafe {
            let rndn = tcod_sys::TCOD_random_new_from_seed(tcod_sys::TCOD_RNG_MT, seed);
            let noise = tcod_sys::TCOD_noise_new(2, 0.7, 0.1, rndn);
            tcod_sys::TCOD_noise_set_type(noise, tcod_sys::TCOD_NOISE_SIMPLEX);
            tcod_sys::TCOD_heightmap_add_fbm(heightmap,
                                             noise,
                                             2.20 * (sx as f32) /
                                                 400.0,
                                             2.20 * (sx as f32) /
                                                 400.0,
                                             0.0,
                                             0.0,
                                             10.0,
                                             1.0,
                                             4.0);
            Self::add_random_hills(heightmap,
                                   (sx, sy),
                                   600,
                                   16.0 * sx as f32 / 200.0,
                                   0.7,
                                   0.3,
                                   rndn);
            Self::dig_random_hills(heightmap,
                                   (sx, sy),
                                   300,
                                   16.0 * sx as f32 / 200.0,
                                   0.6,
                                   0.3,
                                   rndn);
            tcod_sys::TCOD_heightmap_normalize(heightmap, 0.0, 100.0);
            tcod_sys::TCOD_heightmap_rain_erosion(heightmap,
                                                  (sx * sy + 100) as
                                                      i32,
                                                  0.06,
                                                  0.02,
                                                  rndn);
            tcod_sys::TCOD_heightmap_normalize(heightmap, 0.0, 100.0);
        };
        heightmap
    }

    /// Get the unit at the specified position. Returns an optional type.
    pub fn get(&self, pos: Point2D) -> Option<Unit> {
        if self.located_inside(pos) {
            Some(self.map[pos.1][pos.0].clone())
        } else {
            None
        }
    }

    /// Test if the given point is on the World plane.
    pub fn located_inside(&self, pos: Point2D) -> bool {
        return pos.0 < self.map_size.0 && pos.1 < self.map_size.1;
    }

    /// For memory cleanup.
    pub unsafe fn delete_heightmap(&self) {
        tcod_sys::TCOD_heightmap_delete(self.heightmap);
    }

    /// Gets the height of the map at the current point.
    unsafe fn get_height(&self, x: usize, y: usize) -> f32 {
        tcod_sys::TCOD_heightmap_get_value(self.heightmap,
                                           x as i32,
                                           y as i32) *
            THRESHOLD
    }

    /// Gets the liquid purity.
    ///
    /// TODO: Make this based on proximity to dirt and plants, instead
    /// of being random.
    pub fn purity() -> LiquidPurity {
        *rand::thread_rng()
            .choose(&[LiquidPurity::Clean,
                      LiquidPurity::Clear,
                      LiquidPurity::Clear,
                      LiquidPurity::Dirty,
                      LiquidPurity::Dirty,
                      LiquidPurity::Dirty,
                      LiquidPurity::Dirty,
                      LiquidPurity::Muddy,
                      LiquidPurity::Muddy,
                      LiquidPurity::Muddy,
                      LiquidPurity::Murky,
                      LiquidPurity::Pure,
                      LiquidPurity::Sandy,
                      LiquidPurity::Toxic])
            .unwrap()
    }

    pub fn len(&self) -> usize {
        match *self {
            World { ref map, .. } => map.len(),
        }
    }

    pub fn push(&mut self, value: Vec<Unit>) {
        match *self {
            World { ref mut map, .. } => map.push(value),
        }
    }

    fn biome_type_from_noise(point_val: i32) -> BiomeType {
        use self::BiomeType::*;
        match point_val {
            0..10 => Beach,
            10..30 => Jungle,
            30..55 => Forest,
            55..100 => Pasture,
            _ => Forest,
        }
    }

    /// Gets the biome at a certain height.
    pub fn biome_from_noise(point_val: i32,
                            avg_height: f32)
        -> Biome {
        let temp_day = 180f32 - (avg_height * 210f32);
        Biome {
            biome_type: if avg_height < SEA_LEVEL.get() {
                BiomeType::Beach
            } else if temp_day > 100.0 &&
                       avg_height > SEA_LEVEL.get() + 15.0
            {
                BiomeType::Desert
            } else {
                World::biome_type_from_noise(point_val)
            },
            temperature_day_f: temp_day,
            temperature_night_f: 80f32 - (avg_height * 100f32),
            percipitation_chance: point_val as f32,
        }
    }

    fn rock_choice<T: Clone>(list: &[T; 2], rn: f32) -> T {
        if rn < 0.0 {
            list[0].clone()
        } else {
            list[1].clone()
        }
    }

    pub fn is_water(x: Tile) -> bool { matches!(x, Tile::Water(..)) }

    pub fn is_sand(x: Tile) -> bool {
        matches!(x, Tile::Stone(StoneTypes::Soil(SoilTypes::Sandy), _))
    }

    pub fn is_veg(x: Tile) -> bool {
        matches!(x, Tile::Vegetation(..))
    }

    pub fn is_stone(x: Tile) -> bool { matches!(x, Tile::Stone(..)) }

    /// Chooses a type of soil based on adjacency to certain features.
    fn soil_choice(height: i32,
                   adj: Vec<Unit>,
                   seed: u32)
        -> SoilTypes {
        use self::rand::{Rng, SeedableRng, StdRng};
        let seed: &[_] = &[seed as usize];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let falses = (false, false, false, false);
        let adjacencies = adj.iter()
                             .fold(falses, |(w, s, v, sd), x| {
            let ut = x.tiles.borrow();
            if let Some(tx) = ut.get(height as usize) {
                (w || ut.iter().any(|x| Self::is_water(*x)),
                 s || Self::is_sand(*tx),
                 v || Self::is_veg(*tx),
                 sd || Self::is_stone(*tx))
            } else {
                (w || ut.iter().any(|x| Self::is_water(*x)), s, v, sd)
            }
        });
        let (water_adjacent,
             sand_adjacent,
             veg_adjacent,
             sedimentary_adjacent) = adjacencies;
        if water_adjacent {
            SoilTypes::Sandy
        } else if sand_adjacent && rng.gen_range(0, 100) <= 55 {
            SoilTypes::Sandy
        } else if veg_adjacent && water_adjacent ||
                   (water_adjacent && !sand_adjacent)
        {
            SoilTypes::Peaty
        } else if veg_adjacent {
            SoilTypes::Loamy
        } else if sedimentary_adjacent {
            SoilTypes::Silty
        } else if water_adjacent &&
                   (sand_adjacent || sedimentary_adjacent)
        {
            SoilTypes::Clay
        } else {
            SoilTypes::Silty
        }
    }

    /// Chooses a type of rock based on the World's stone_noise FBM and height.
    pub fn rock_type(&self, (x, y): Point2D, height: isize) -> Tile {
        let rn = self.stone_vein_noise
                     .get_fbm(&mut [x as f32,
                                    y as f32,
                                    height as f32],
                              2) * 100.0;
        let sedimentary = &[SedimentaryRocks::Conglomerate,
                            SedimentaryRocks::Limestone];
        let igneous = &[IgneousRocks::Obsidian, IgneousRocks::Basalt];
        let metamorphic = &[MetamorphicRocks::Marble,
                            MetamorphicRocks::Gneiss];
        Tile::Stone(
            // Stone type
            if height < 2 {
                let v = World::rock_choice(igneous, rn);
                StoneTypes::Igneous(v.clone())
            } else if height as f32 <= SEA_LEVEL.get() - 1.0 {
                let v = World::rock_choice(metamorphic, rn);
                StoneTypes::Metamorphic(v.clone())
            } else if height as f32 <= SEA_LEVEL.get() + 3.25 {
                let v = World::rock_choice(sedimentary, rn);
                StoneTypes::Sedimentary(v.clone())
            } else {
                let v = World::rock_choice(metamorphic, rn);
                StoneTypes::Metamorphic(v.clone())
            },

            // State
            if height == 0 {
                if rn < 0.0 {
                    State::Liquid
                } else {
                    State::Solid
                }
            } else {
                State::Solid
            },
        )
    }

    /// Gets the correct vegitation based on the heightmap's height and random selection.
    pub fn get_vegetation(noise: &Noise,
                          (x, y): Point2D,
                          biome: Biome)
        -> Tile {
        let vn = noise.get_fbm(&mut [x as f32, y as f32], 1) * 100.0;
        let veg_levels = vec![[VegType::Bluegrass,
                               VegType::Bentgrass,
                               VegType::Ryegrass],
                              [VegType::Dandelion,
                               VegType::Chickweed,
                               VegType::Dandelion],
                              [VegType::Redbud,
                               VegType::Rhododendron,
                               VegType::BroomShrub],
                              [VegType::Crabapple,
                               VegType::Redbud,
                               VegType::Crabapple],
                              [VegType::Pine,
                               VegType::Crabapple,
                               VegType::Pine],
                              [VegType::Redwood,
                               VegType::Pine,
                               VegType::Banyon]];
        let mut trng = rand::thread_rng();
        let (vopts, height) = if vn < 0.0 {
            (&veg_levels[0], 1)
        } else if vn < 2.0 {
            (&veg_levels[1], trng.gen_range(1, 3))
        } else if vn < 15.0 {
            (&veg_levels[2], trng.gen_range(4, 6))
        } else if vn < 20.0 {
            (&veg_levels[3], trng.gen_range(6, 9))
        } else if vn < 40.0 {
            (&veg_levels[4], trng.gen_range(7, 10))
        } else if vn < 100.0 {
            (&veg_levels[5], trng.gen_range(10, 20))
        } else {
            (&veg_levels[5], vn as i32)
        };

        if let Some(v) = trng.choose(
            &vopts.iter()
                  .filter(|v| biome.survives(**v))
                  .collect::<Vec<_>>()
                [0..],
        )
        {
            Tile::Vegetation(*v.clone(), height, State::Solid)
        } else {
            Tile::Vegetation(VegType::Dandelion, 1, State::Solid)
        }
    }

    pub fn location_z(&self, pos: Point2D) -> usize {
        let loc = self.get(pos);
        loc.map(|u| {
            let t = u.tiles.borrow();
            t.iter()
             .enumerate()
             .find(|&t| *t.1 == Tile::Empty)
             .map(|(i, _)| i)
             .unwrap_or(t.len()
                         .checked_sub(1)
                         .unwrap_or(1000000))
        })
           .unwrap_or(10000000)
    }

    pub fn location_z_from_to(&self,
                              from: usize,
                              to: Point2D)
        -> usize {
        let loc = self.get(to);
        let mut openings =
            loc.map(|u| {
                u.tiles
                 .borrow()
                 .iter()
                 .enumerate()
                 .flat_map(|(i, t)| if t.solid() {
                               None
                           } else {
                               Some(i)
                           })
                 .collect::<Vec<_>>()
            })
               .unwrap_or(vec![1000]);
        openings.sort_by(|a, b| b.cmp(a));

        openings.iter().fold(
            1000,
            |best, new| if (*new as i32 - from as i32).abs() <
                (best as i32 - from as i32).abs()
            {
                *new
            } else {
                best
            },
        )
    }
}

/// Handles general time:
/// * calendar date
/// * time of day (fuzzy)
/// * absolute time (clock)
/// * days since universe start
pub struct TimeHandler {
    pub calendar: Calendar,
    pub time_of_day: Time,
    pub clock: Clock,
    pub days: usize,
}

/// Handles overall world state:
///
/// * the 3D screen location and cursor position
/// * the organisms
/// * meta info about the map and the map itself
/// * and time, including dates and clock time.
///
/// WorldState also handles generating a new map, which, for
/// performance reasons, is not requred on the creation of the struct,
/// instead relying on after-the-fact linking.
pub struct WorldState {
    pub screen: (i32, i32),
    pub cursor: (i32, i32),
    pub level: i32,
    pub commands: Vec<Order>,
    pub map: Option<World>,
    pub highest_level: usize,
    pub time: TimeHandler,
}

impl WorldState {
    fn update_time(&mut self, time: usize, dt: usize) {
        self.time.clock.update_deltatime(dt);
        self.time.time_of_day = Time::from_clock_time(&self.time
                                                           .clock);
        if self.time.time_of_day == Time::Midnight {
            self.time.clock.time = (0, 0);
            self.time.days += 1;
            self.time
                .calendar
                .update_to_day(self.time.days, &self.time.clock);
        }
    }

    fn update_life(&mut self, time: usize) {
        if let Some(ref mut world) = self.map {
            for i in 0..world.life.len() {
                let modifier = if i % 2 == 0 { 2 } else { 3 };
                if time % modifier == 0 {
                    let res = {
                        let mut actor = world.life[i].borrow_mut();
                        actor.execute_mission(world)
                    };
                    match res {
                        MissionResult::Die => world.kill(i),
                        MissionResult::Kill(i) => world.kill(i),
                        MissionResult::RemoveItem(pnt) => {
                            let unit = &world.map[pnt.1][pnt.0];
                            let mut tiles = unit.tiles.borrow_mut();
                            tiles[pnt.2] = Tile::Empty;
                        }
                        MissionResult::ReplaceItem(pnt, item) => {
                            let unit = &world.map[pnt.1][pnt.0];
                            let mut tiles = unit.tiles.borrow_mut();
                            tiles[pnt.2] = Tile::Item(item);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    /// Updates world time and then deligates to the physics engine.
    pub fn update(&mut self, time: usize, dt: usize) {
        self.update_time(time, dt);
        self.update_life(time);
        //physics::run(self, dt);
    }

    /// Add a world map and update its meta layer.
    pub fn add_map(&mut self, world: World) {
        let max = world.map
                       .iter()
                       .flat_map(|r| {
            r.iter()
             .map(|unit| unit.tiles.borrow().len())
             .max()
        })
                       .max();
        self.map = Some(world);
        self.highest_level = max.unwrap_or(30);
        self.map
            .as_mut()
            .unwrap()
            .generate_life();
    }

    /// Create a new WorldState, loaded with sensable defaults.
    pub fn new() -> WorldState {
        let clock = Clock { time: (12, 30) };
        WorldState {
            commands: vec![],
            screen: (0, 0),
            level: 31,
            highest_level: 0,
            cursor: (0, 0),
            time: TimeHandler {
                days: 36512,
                calendar: Calendar::new(12, 6, 100, &clock),
                time_of_day: Time::Morning,
                clock: clock,
            },
            map: None,
        }
    }
}
