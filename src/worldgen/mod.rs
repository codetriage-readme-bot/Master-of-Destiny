extern crate rand;
extern crate tcod_sys;

use life;

use self::rand::Rng;
use std::cell::RefCell;
use std::cmp;
use std::ops::{Index, Range};

use tcod::map;
use tcod::noise::{Noise, NoiseType};
use tcod::random;

pub mod terrain;
use self::terrain::*;

use physics;
use physics::liquid;
use physics::stone;

use time::*;

pub fn clamp<T: Ord>(value: T, max: T, min: T) -> T {
    cmp::min(max, cmp::max(min, value))
}

macro_rules! matches {
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
}


fn restricted_from_tile(tile: Tile) -> RestrictedTile {
    match tile {
        Tile::Stone(a, b) => RestrictedTile::Stone(a, b),
        Tile::Vegitation(a, b, c) => {
            RestrictedTile::Vegitation(a, b, c)
        }
        _ => {
            panic!("Type error. Cannot allow restricted type of kind {:?}",
                   tile)
        }
    }
}

fn can_be_restricted(tile: Tile) -> bool {
    match tile {
        Tile::Stone(..) => true,
        Tile::Vegitation(..) => true,
        _ => false,
    }
}

pub fn strict_adjacent((x, y): (usize, usize))
    -> Vec<(usize, usize)> {
    vec![(x + 1, y),
         (x.checked_sub(1).unwrap_or(0), y),
         (x, y.checked_sub(1).unwrap_or(0)),
         (x, y + 1)]
}

pub fn weak_adjacent(p: (usize, usize)) -> Vec<(usize, usize)> {
    let mut v = strict_adjacent(p);
    v.push(p);
    v
}

// A unit is a 1x1 cross section of the layered world, including a ref
// to the biome its part of.
// Each tile is 1 foot tall.
#[derive(Clone)]
pub struct Unit {
    pub biomes: Vec<Biome>,
    pub tiles: Vec<Tile>,
}

// World contains the current state of the PHYSICAL world
pub struct World {
    pub map: Vec<Vec<Unit>>,
    heightmap: *mut tcod_sys::TCOD_heightmap_t,
    vegetation_noise: Noise,
    stone_vein_noise: Noise,
}

impl Index<usize> for World {
    type Output = Vec<Unit>;
    fn index(&self, location: usize) -> &Vec<Unit> {
        match self {
            &World { ref map, .. } => &map[location],
        }
    }
}

impl Index<Range<usize>> for World {
    type Output = [Vec<Unit>];
    fn index(&self, location: Range<usize>) -> &[Vec<Unit>] {
        match self {
            &World { ref map, .. } => &map[location],
        }
    }
}

fn random_hill_operation<F>(heightmap: *mut tcod_sys::TCOD_heightmap_t,
                            (hmw, hmh): (usize, usize),
                            num_hills: i32,
                            base_radius: f32,
                            radius: f32,
                            height: f32,
                            rndn: tcod_sys::TCOD_random_t,
                            operation: &F)
where
    F: Fn(*mut tcod_sys::TCOD_heightmap_t,
       f32,
       f32,
       f32,
       f32),
{
    unsafe {
        for _ in 0..num_hills {
            let radius =
                tcod_sys::TCOD_random_get_float(rndn,
                                                base_radius *
                                                    (1.0 - radius),
                                                base_radius *
                                                    (1.0 + radius));
            let xh = tcod_sys::TCOD_random_get_int(rndn,
                                                   0,
                                                   hmw as i32 - 1);
            let yh = tcod_sys::TCOD_random_get_int(rndn,
                                                   0,
                                                   hmh as i32 - 1);
            operation(heightmap,
                      xh as f32,
                      yh as f32,
                      radius,
                      height);
        }
    }
}

fn add_random_hills(hm: *mut tcod_sys::TCOD_heightmap_t,
                    sz: (usize, usize),
                    nh: i32,
                    br: f32,
                    r: f32,
                    h: f32,
                    n: tcod_sys::TCOD_random_t) {
    random_hill_operation(hm,
                          sz,
                          nh,
                          br,
                          r,
                          h,
                          n,
                          &|a, b, c, d, e| unsafe { tcod_sys::TCOD_heightmap_add_hill(a, b, c, d, e) });
}

fn dig_random_hills(hm: *mut tcod_sys::TCOD_heightmap_t,
                    sz: (usize, usize),
                    nh: i32,
                    br: f32,
                    r: f32,
                    h: f32,
                    n: tcod_sys::TCOD_random_t) {
    random_hill_operation(hm,
                          sz,
                          nh,
                          br,
                          r,
                          h,
                          n,
                          &|a, b, c, d, e| unsafe { tcod_sys::TCOD_heightmap_dig_hill(a, b, c, d, e) });
}

const THRESHOLD: f32 = 0.5;
const SEA_LEVEL: f32 = 12.0;
const VEG_THRESHOLD: f32 = 200.0;
const RAMP_THRESHOLD: f32 = 0.015;
impl World {
    pub fn new(size: (usize, usize), seed: u32) -> World {
        println!("Generating seed from {}", seed);

        // Vegitation
        let vnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT,
                                               seed))
            .init();

        let (sx, sy) = size;

        // And here we see a small C hiding in the maw of a Rust. <low-level>
        let heightmap = unsafe {
            tcod_sys::TCOD_heightmap_new(sx as i32, sy as i32)
        };
        unsafe {
            let rndn =
                tcod_sys::TCOD_random_new_from_seed(tcod_sys::TCOD_RNG_MT,
                                                    seed);
            let noise = tcod_sys::TCOD_noise_new(2, 0.7, 0.1, rndn);
            tcod_sys::TCOD_noise_set_type(noise,
                                          tcod_sys::TCOD_NOISE_PERLIN);
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
                                             2.05);
            add_random_hills(heightmap,
                             size,
                             600,
                             16.0 * sx as f32 / 200.0,
                             0.7,
                             0.3,
                             rndn);
            dig_random_hills(heightmap,
                             size,
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
        }

        // </low-level>

        let mut world: World = World {
            map: vec![],
            heightmap: heightmap,
            vegetation_noise: vnoise,
            stone_vein_noise: Noise::init_with_dimensions(3)
                .lacunarity(0.43)
                .hurst(-0.9)
                .noise_type(NoiseType::Perlin)
                .random(random::Rng::new_with_seed(random::Algo::MT,
                                                   seed))
                .init(),
        };
        for y in 0..sy {
            let mut line = vec![];
            for x in 0..sx {
                let height = unsafe { world.get_height(x, y) };
                let slope = unsafe {
                    tcod_sys::TCOD_heightmap_get_slope(heightmap,
                                                       x as i32,
                                                       y as i32)
                };
                let mut tiles: Vec<Tile> = vec![];
                let mut biomes: Vec<Biome> = vec![];
                if height <= SEA_LEVEL {
                    let dist = height as isize + 7;
                    for z in 0..dist {
                        tiles.push(Tile::Water(World::purity(),
                                               State::Liquid,
                                               (dist - z) as i32));
                    }
                } else {
                    let adj = strict_adjacent((x, y))
                        .iter()
                        .map(|&(x, y)| {
                            let list = if y > world.len() {
                                vec![]
                            } else if y == world.len() {
                                line.clone()
                            } else {
                                world[y].clone()
                            };

                            if x >= list.len() {
                                Tile::Empty
                            } else {
                                if height as usize >=
                                    list[x].tiles.len()
                                {
                                    Tile::Empty
                                } else {
                                    list[x].tiles[height as usize]
                                }
                            }
                        })
                        .collect::<Vec<_>>();
                    let water_adjacent =
                        adj.iter()
                           .find(|x| match **x {
                                     Tile::Water(..) => true,
                                     _ => false,
                                 })
                           .is_some();

                    for z in 0..(height as isize - 1) {
                        biomes.push(world.biome_from_height(z));
                        tiles.push(world.rock_type(adj.clone(),
                                                   (x, y),
                                                   z));
                    }
                    if height <= VEG_THRESHOLD && !water_adjacent {
                        match world.get_vegetation((x, y)) {
                            Tile::Vegitation(a, height, b) => {
                                for z in 0..height {
                                    if z >= height / 3 {
                                        tiles.push(Tile::Vegitation(a,
                                                                    height -
                                                                        z,
                                                                    b));
                                    } else {
                                        tiles.push(Tile::Vegitation(VegTypes::Treetrunk,
                                                                    1,
                                                                    State::Solid));
                                    }
                                }
                            }
                            _ => {
                                panic!("Don't panic? Now is the perfect time to panic!")
                            }
                        }
                    }
                    let tile = match tiles.len() {
                        0 => None,
                        n => Some(tiles[n - 1]),
                    };
                    if tile.is_some() {
                        let tile = tile.unwrap();
                        let last = (tiles.len() - 1) as usize;
                        if can_be_restricted(tile) {
                            let r =
                                restricted_from_tile(tile.clone());
                            tiles[last] = if slope < -RAMP_THRESHOLD {
                                Tile::Ramp(r, Slope::Down)
                            } else if slope > RAMP_THRESHOLD {
                                Tile::Ramp(r, Slope::Up)
                            } else {
                                tile
                            }
                        }
                    }
                }
                line.push(Unit {
                              tiles: tiles,
                              biomes: biomes,
                          });
            }
            world.push(line);
        }

        println!("Done world generation!");
        world
    }

    pub unsafe fn delete_heightmap(&self) {
        tcod_sys::TCOD_heightmap_delete(self.heightmap);
    }

    unsafe fn get_height(&self, x: usize, y: usize) -> f32 {
        tcod_sys::TCOD_heightmap_get_value(self.heightmap,
                                           x as i32,
                                           y as i32) *
            THRESHOLD
    }

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

    pub fn biome_from_height(&self, height: isize) -> Biome {
        Biome {
            biome_type: BiomeType::Pasture,
            temperature_day_f: 75,
            temperature_night_f: 52,
            percipitation_chance: 70,
        }
    }

    fn rock_choice<T: Clone>(list: &[T; 2], rn: f32) -> T {
        if rn < 0.0 {
            list[0].clone()
        } else {
            list[1].clone()
        }
    }

    fn soil_choice(adj: Vec<Tile>) -> SoilTypes {
        let (water_adjacent, sand_adjacent, veg_adjacent, sedimentary_adjacent) = adj.iter()
                                .fold((false, false, false, false),
                                      |(w, s, v, sd), x|
                                      {
                                          (w || matches!(*x, Tile::Water(..)),
                                           s || matches!(*x, Tile::Stone(StoneTypes::Soil(SoilTypes::Sandy), _)),
                                           v || matches!(*x, Tile::Vegitation(..)),
                                           sd || matches!(*x, Tile::Stone(..)))
                                      });
        if water_adjacent || sand_adjacent {
            SoilTypes::Sandy
        } else if veg_adjacent && water_adjacent ||
                   (water_adjacent && !sand_adjacent)
        {
            SoilTypes::Peaty
        } else if veg_adjacent {
            *rand::thread_rng()
                .choose(&[SoilTypes::Loamy, SoilTypes::Peaty])
                .unwrap()
        } else if sedimentary_adjacent {
            *rand::thread_rng()
                .choose(&[SoilTypes::Clay, SoilTypes::Silty])
                .unwrap()
        } else if water_adjacent &&
                   (sand_adjacent || sedimentary_adjacent)
        {
            SoilTypes::Clay
        } else {
            *rand::thread_rng()
                .choose(&[SoilTypes::Clay, SoilTypes::Loamy])
                .unwrap()
        }
    }

    pub fn rock_type(&self,
                     adj: Vec<Tile>,
                     (x, y): (usize, usize),
                     height: isize)
        -> Tile {
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
        // [Sedimentary]->[Soil]->[Metamorphic]->[Igneous]
        Tile::Stone(// Stone type
                    if height < 10 {
                        let v = World::rock_choice(igneous, rn);
                        StoneTypes::Igneous(v.clone())
                    } else if height as f32 <= SEA_LEVEL - 4.0 {
                        let v = World::rock_choice(metamorphic, rn);
                        StoneTypes::Metamorphic(v.clone())
                    } else if height as f32 <= SEA_LEVEL + 3.0 {
                        let v = World::soil_choice(adj);
                        StoneTypes::Soil(v.clone())
                    } else if height as f32 <= SEA_LEVEL + 13.0 {
                        let v = World::rock_choice(sedimentary, rn);
                        StoneTypes::Sedimentary(v.clone())
                    } else {
                        let v = World::rock_choice(metamorphic, rn);
                        StoneTypes::Metamorphic(v.clone())
                    },

                    // State
                    if height < 3 {
                        if rn < 0.0 {
                            State::Liquid
                        } else {
                            State::Solid
                        }
                    } else {
                        State::Solid
                    })
    }

    pub fn get_vegetation(&self, (x, y): (usize, usize)) -> Tile {
        let vn = self.vegetation_noise
                     .get_fbm(&mut [x as f32, y as f32], 1) *
            100.0;
        let veg_levels = vec![[VegTypes::Bluegrass,
                               VegTypes::Bentgrass,
                               VegTypes::Ryegrass],
                              [VegTypes::Dandelion,
                               VegTypes::Chickweed,
                               VegTypes::Dandelion],
                              [VegTypes::Redbud,
                               VegTypes::Rhododendron,
                               VegTypes::BroomShrub],
                              [VegTypes::Crabapple,
                               VegTypes::Redbud,
                               VegTypes::Crabapple],
                              [VegTypes::Pine,
                               VegTypes::Crabapple,
                               VegTypes::Pine],
                              [VegTypes::Redwood,
                               VegTypes::Pine,
                               VegTypes::Banyon]];
        let mut trng = rand::thread_rng(); // I don't know why this should be mutable!
        let veg = if vn < 0.0 {
            (trng.choose(&veg_levels[0]), 1)
        } else if vn < 2.0 {
            (trng.choose(&veg_levels[1]), trng.gen_range(1, 3))
        } else if vn < 15.0 {
            (trng.choose(&veg_levels[2]), trng.gen_range(4, 6))
        } else if vn < 20.0 {
            (trng.choose(&veg_levels[3]), trng.gen_range(6, 9))
        } else if vn < 40.0 {
            (trng.choose(&veg_levels[4]), trng.gen_range(7, 10))
        } else if vn < 100.0 {
            (trng.choose(&veg_levels[5]), trng.gen_range(10, 20))
        } else {
            (Some(&veg_levels[5][0]), vn as i32)
        };

        let v = *veg.0.unwrap();
        Tile::Vegitation(v.clone(), veg.1, State::Solid)
    }
}

pub struct WorldState {
    pub map_size: (usize, usize),
    pub screen: (i32, i32),
    pub cursor: (i32, i32),
    pub level: i32,
    pub life: Vec<Box<life::Living>>,
    pub map: Option<RefCell<World>>,
    pub highest_level: usize,
    pub time_of_day: Time,
    pub tcod_map: map::Map,
}

pub const CYCLE_LENGTH: usize = 100;
impl WorldState {
    pub fn update(&mut self, time: usize, dt: usize) {
        self.time_of_day = calculate_time_of_day(time, CYCLE_LENGTH);
        physics::run(self, dt);
    }
    pub fn add_map(&mut self, world: World) {
        let f = |vec: &Vec<Unit>| -> Option<usize> {
            vec.iter()
               .fold(None, |max, unit: &Unit| match max {
                None => Some(unit.tiles.len()),
                q @ Some(_) => {
                    if q.unwrap() < unit.tiles.len() {
                        Some(unit.tiles.len())
                    } else {
                        q
                    }
                }
            })
        };

        // find the highest height in this list of rows. Same as above.
        let toplevel = world.map
                            .iter()
                            .fold(None, |max, ref vec| match max {
            None => f(vec),
            q @ Some(_) => {
                match f(vec) {
                    None => q,
                    z @ Some(_) => {
                        if q.unwrap() < z.unwrap() { z } else { q }
                    }
                }
            }
        });
        self.map = Some(RefCell::new(world));
        self.highest_level = toplevel.unwrap();
    }
    pub fn new(s: (usize, usize)) -> WorldState {
        WorldState {
            screen: (0, 0),
            level: 31,
            highest_level: 0,
            cursor: (0, 0),
            time_of_day: Time::Morning,
            life: vec![],
            map: None,
            map_size: s,
            tcod_map: map::Map::new(s.0 as i32, s.1 as i32),
        }
    }
}
