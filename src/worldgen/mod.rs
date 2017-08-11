extern crate rand;
extern crate tcod_sys;

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Index, Range};
use std::rc::Rc;

use tcod::noise::{Noise, NoiseType};
use tcod::random;

pub mod terrain;
use self::rand::Rng;
use self::terrain::*;

use life;
use life::{Living, Mission, MissionResult};
use physics;
use time::{Calendar, Clock, Time};
use utils::{Point2D, strict_adjacent};

macro_rules! matches {
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
}

/// Returns the Some() of the restricted version of a Tile if it can be restricted, if not, returns None.
fn restricted_from_tile(tile: Tile) -> Option<RestrictedTile> {
    match tile {
        Tile::Stone(a, b) => Some(RestrictedTile::Stone(a, b)),
        Tile::Vegitation(a, b, c) => {
            Some(RestrictedTile::Vegitation(a, b, c))
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

/// Keeps track of the current animation frame and all the rest of the
/// animation frames. These frames are stored relative to the
/// beginning of the tiles in the tileset.
#[derive(Clone)]
pub struct FrameAssoc {
    pub current: usize,
    pub all: Vec<usize>,
}
/// A mapping of a unique string (based on the type of thing animated)
/// to its FrameAssoc.
pub type Frames = HashMap<String, RefCell<FrameAssoc>>;

/// Keeps track of the state of they physical world, including:
/// * the heightmap
/// * the vegitation and stone noise
/// * the animation state
/// * the map size and unit map
pub struct World {
    heightmap: *mut tcod_sys::TCOD_heightmap_t,
    vegetation_noise: Noise,
    stone_vein_noise: Noise,
    pub map_size: Point2D,
    pub frames: Frames,
    pub map: Vec<Vec<Rc<Unit>>>,
}

impl Index<usize> for World {
    type Output = Vec<Rc<Unit>>;
    fn index(&self, location: usize) -> &Self::Output {
        &self.map[location]
    }
}

impl Index<Range<usize>> for World {
    type Output = [Vec<Rc<Unit>>];
    fn index(&self, location: Range<usize>) -> &Self::Output {
        &self.map[location]
    }
}

const THRESHOLD: f32 = 0.5;
const SEA_LEVEL: f32 = 12.0;
const VEG_THRESHOLD: f32 = 200.0;
const RAMP_THRESHOLD: f32 = 0.015;
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
        println!("Generating seed from {}", seed);

        // Vegitation
        let vnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT,
                                               seed))
            .init();

        let mut world: World = World {
            map_size: size,
            heightmap: Self::generate_heightmap(size, seed),
            map: vec![],
            vegetation_noise: vnoise,
            stone_vein_noise: Noise::init_with_dimensions(3)
                .lacunarity(0.43)
                .hurst(-0.9)
                .noise_type(NoiseType::Perlin)
                .random(random::Rng::new_with_seed(random::Algo::MT,
                                                   seed))
                .init(),
            frames: [("Water".to_string(),
                      FrameAssoc {
                          current: 0,
                          all: vec![16, 32, 33, 34, 35, 36],
                      })]
                    .iter()
                    .cloned()
                    .map(|(x, y)| (x, RefCell::new(y)))
                    .collect(),
        };
        world.map = Self::map_from(size, &world);
        world
    }

    /// Creates a vector of animals based on biome and height.
    pub fn generate_life(&self) -> Vec<RefCell<Box<Living>>> {
        vec![]
    }

    /// Generates a new unit map for World from the given (incomplete) World.
    fn map_from((sx, sy): Point2D, ws: &World) -> Vec<Vec<Rc<Unit>>> {
        let mut world: Vec<Vec<Rc<Unit>>> = vec![];
        for y in 0..sy {
            let mut line = vec![];
            for x in 0..sx {
                let height = unsafe { ws.get_height(x, y) };
                let slope = unsafe {
                    tcod_sys::TCOD_heightmap_get_slope(ws.heightmap,
                                                       x as i32,
                                                       y as i32)
                };
                let mut tiles: Vec<Tile> = vec![];
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
                                let lt = list[x].tiles.borrow();
                                if height as usize >= lt.len() {
                                    Tile::Empty
                                } else {
                                    lt[height as usize]
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
                        tiles.push(ws.rock_type(adj.clone(),
                                                (x, y),
                                                z));
                    }
                    if height <= VEG_THRESHOLD && !water_adjacent {
                        match ws.get_vegetation((x, y)) {
                            Tile::Vegitation(a, height, b) => {
                                for z in 0..height {
                                    tiles.push(if z >= height / 3 {
                                        Tile::Vegitation(a,
                                                         height - z,
                                                         b)
                                    } else {
                                        Tile::Vegitation(
                                            VegTypes::Treetrunk,
                                            1,
                                            State::Solid)
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                    let tile = match tiles.len() {
                        0 => None,
                        n => Some(tiles[n - 1]),
                    };
                    if tile.is_some() {
                        let tile = tile.unwrap();
                        let last = (tiles.len() - 1) as usize;
                        if let Some(r) =
                            restricted_from_tile(tile.clone())
                        {
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
                line.push(Rc::new(Unit {
                                      tiles: RefCell::new(tiles),
                                      biome: None,
                                  }));
            }
            world.push(line);
        }
        world
    }

    /// A general method for dealing with generating random hills of a limited size, position, and height.
    fn random_hill_operation<F>(heightmap: *mut tcod_sys::TCOD_heightmap_t,
                            (hmw, hmh): Point2D,
                            num_hills: i32,
                            base_radius: f32,
                            radius: f32,
                            height: f32,
                            rndn: tcod_sys::TCOD_random_t,
                            operation: &F)
where F: Fn(*mut tcod_sys::TCOD_heightmap_t,
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
        };
        heightmap
    }

    /// Get the unit at the specified position. Returns an optional type.
    pub fn get(&self, pos: Point2D) -> Option<Rc<Unit>> {
        if self.located_inside(pos) {
            Some(self.map[pos.1][pos.0].clone())
        } else {
            None
        }
    }

    /// Test if the given point is on the World plane.
    pub fn located_inside(&self, pos: Point2D) -> bool {
        return pos.0 >= 0 && pos.0 < self.map_size.0 &&
            pos.1 >= 0 && pos.1 < self.map_size.1;
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

    pub fn push(&mut self, value: Vec<Rc<Unit>>) {
        match *self {
            World { ref mut map, .. } => map.push(value),
        }
    }

    fn biome_type_from_noise(point_val: f32) -> BiomeType {
        use self::BiomeType::*;
        match point_val as i32 * 100 {
            0..10 => Beach,
            10..25 => Jungle,
            25..55 => Forest,
            55..100 => Pasture,
            _ => Forest,
        }
    }

    /// Gets the biome at a certain height.
    pub fn biome_from_noise(point_val: f32,
                            avg_height: f32)
        -> Biome {
        Biome {
            biome_type: World::biome_type_from_noise(point_val),
            temperature_day_f: 100f32 - (avg_height * 100f32),
            temperature_night_f: 80f32 - (avg_height * 100f32),
            percipitation_chance: point_val,
        }
    }

    fn rock_choice<T: Clone>(list: &[T; 2], rn: f32) -> T {
        if rn < 0.0 {
            list[0].clone()
        } else {
            list[1].clone()
        }
    }

    /// Chooses a type of soil based on adjacency to certain features.
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

    /// Chooses a type of rock based on the World's stone_noise FBM and height.
    pub fn rock_type(&self,
                     adj: Vec<Tile>,
                     (x, y): Point2D,
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

    /// Gets the correct vegitation based on the heightmap's height and random selection.
    pub fn get_vegetation(&self, (x, y): Point2D) -> Tile {
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
    pub life: Vec<RefCell<Box<Living>>>,
    pub map: Option<World>,
    pub highest_level: usize,
    pub time: TimeHandler,
}

impl WorldState {
    /// Updates world time and then deligates to the physics engine.
    pub fn update(&mut self, time: usize, dt: usize) {
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
        if let Some(ref world) = self.map {
            let map = &world.map;
            let mut kills = vec![];
            for i in 0..self.life.len() {
                let mut actor = self.life[i].borrow_mut();
                match actor.execute_mission(self) {
                    MissionResult::Kill(i) => {
                        kills.push(i);
                    }
                    MissionResult::RemoveItem(pnt) => {
                        let unit = &map[pnt.1][pnt.0];
                        let mut tiles = unit.tiles.borrow_mut();
                        tiles[pnt.2] = Tile::Empty;
                    }
                    MissionResult::ReplaceItem(pnt, item) => {
                        let unit = &map[pnt.1][pnt.0];
                        let mut tiles = unit.tiles.borrow_mut();
                        tiles[pnt.2] = Tile::Item(item);
                    }
                    _ => {}
                }
            }
        }
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
        if let Some(ref map) = self.map {
            self.life = map.generate_life();
        }
    }

    /// Create a new WorldState, loaded with sensable defaults.
    pub fn new(s: Point2D) -> WorldState {
        let clock = Clock { time: (12, 30) };
        WorldState {
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
            life: vec![],
            map: None,
        }
    }

    pub fn can_move<'a>(
        &self,
        actor: &life::animal::Animal<'a>)
        -> impl FnMut((i32, i32), (i32, i32)) -> f32 {
        move |from: (i32, i32), to: (i32, i32)| -> f32 { 1.0 }
    }

    pub fn location_z(&self, pos: Point2D) -> usize {
        if let Some(ref map) = self.map {
            let loc = map.get(pos);
            loc.map(|u| {
                let t = u.tiles.borrow();
                t.iter()
                 .enumerate()
                 .find(|&t| *t.1 == Tile::Empty)
                 .map(|(i, _)| i)
                 .unwrap_or(t.len())
            })
               .unwrap_or(0)
        } else {
            0
        }
    }
}
