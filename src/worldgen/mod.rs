extern crate rand;
extern crate tcod_sys;

use draw::{Describe, DrawChar};
use life;

use self::rand::Rng;
use std::cmp;
use std::ops::{Index, Range};

use tcod::RootConsole;
use tcod::chars;
use tcod::colors::Color;
use tcod::console::{BackgroundFlag, Console};
use tcod::map;
use tcod::noise::{Noise, NoiseType};
use tcod::random;


pub fn clamp<T: Ord>(value: T, max: T, min: T) -> T {
    cmp::min(max, cmp::max(min, value))
}

fn restricted_from_tile(tile: Tile) -> RestrictedTile {
    match tile {
        Tile::Stone(a, b) => RestrictedTile::Stone(a, b),
        Tile::Vegitation(a, b, c) => RestrictedTile::Vegitation(a, b, c),
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

/////// ROCK
// Possible igneous rock kinds
#[derive(Debug, Copy, Clone)]
pub enum IgneousRocks {
    Obsidian,
    Basalt,
}

impl Describe for IgneousRocks {
    fn describe(&self) -> String {
        match self {
            &IgneousRocks::Obsidian => "Igneous obisidian".to_string(),
            &IgneousRocks::Basalt => "Igneous basalt".to_string(),
        }
    }
}

impl DrawChar for IgneousRocks {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &IgneousRocks::Obsidian => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK1,
                                 Color::new(12, 12, 12),
                                 Color::new(2, 2, 2))
            }
            &IgneousRocks::Basalt => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK2,
                                 Color::new(20, 20, 12),
                                 Color::new(2, 2, 2))
            }
        };
    }
}

// Possible metamorphic rock kinds
#[derive(Debug, Copy, Clone)]
pub enum MetamorphicRocks {
    Gneiss,
    Marble,
}

impl Describe for MetamorphicRocks {
    fn describe(&self) -> String {
        match self {
            &MetamorphicRocks::Gneiss => "Metamorphic gneiss".to_string(),
            &MetamorphicRocks::Marble => "Metamorphic marble".to_string(),
        }
    }
}

impl DrawChar for MetamorphicRocks {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &MetamorphicRocks::Gneiss => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK2,
                                 Color::new(125, 85, 62),
                                 Color::new(2, 2, 2))
            }
            &MetamorphicRocks::Marble => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK1,
                                 Color::new(250, 250, 250),
                                 Color::new(12, 12, 12))
            }
        };
    }
}

// Possible Sedimentary rock kinds
#[derive(Debug, Copy, Clone)]
pub enum SedimentaryRocks {
    Limestone,
    Conglomerate,
}

impl Describe for SedimentaryRocks {
    fn describe(&self) -> String {
        match self {
            &SedimentaryRocks::Limestone => {
                "Sedimentary limestone".to_string()
            }
            &SedimentaryRocks::Conglomerate => {
                "Sedimentary conglomerate".to_string()
            }
        }
    }
}

impl DrawChar for SedimentaryRocks {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &SedimentaryRocks::Limestone => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK3,
                                 Color::new(125, 85, 62),
                                 Color::new(255, 255, 255))
            }
            &SedimentaryRocks::Conglomerate => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::BLOCK2,
                                 Color::new(130, 100, 50),
                                 Color::new(200, 200, 200))
            }
        };
    }
}

// Soil types
#[derive(Debug, Copy, Clone)]
pub enum SoilTypes {
    Clay,
    Sandy,
    Silty,
    Peaty,
    Chalky,
    Loamy,
}

impl Describe for SoilTypes {
    fn describe(&self) -> String {
        match self {
            &SoilTypes::Clay => "Clay soil".to_string(),
            &SoilTypes::Sandy => "Sandy soil".to_string(),
            &SoilTypes::Silty => "Silty soil".to_string(),
            &SoilTypes::Peaty => "Peaty soil".to_string(),
            &SoilTypes::Chalky => "Chalky soil".to_string(),
            &SoilTypes::Loamy => "Loamy soil".to_string(),
        }
    }
}

impl DrawChar for SoilTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &SoilTypes::Clay => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(167, 107, 41),
                                 Color::new(191, 100, 35))
            }
            &SoilTypes::Sandy => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(167, 107, 41),
                                 Color::new(191, 100, 35))
            }
            &SoilTypes::Silty => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(133, 126, 108),
                                 Color::new(99, 94, 80))
            }
            &SoilTypes::Peaty => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(159, 145, 95),
                                 Color::new(119, 108, 71))
            }
            &SoilTypes::Chalky => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(219, 233, 237),
                                 Color::new(143, 186, 199))
            }
            &SoilTypes::Loamy => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '=',
                                 Color::new(86, 59, 56),
                                 Color::new(64, 44, 41))
            }   
        }
    }
}

// Stone types (SCIENCE!)
#[derive(Debug, Copy, Clone)]
pub enum StoneTypes {
    Sedimentary(SedimentaryRocks),
    Igneous(IgneousRocks),
    Metamorphic(MetamorphicRocks),
    Soil(SoilTypes),
}

impl Describe for StoneTypes {
    fn describe(&self) -> String {
        match self {
            &StoneTypes::Sedimentary(v) => v.describe(),
            &StoneTypes::Igneous(v) => v.describe(),
            &StoneTypes::Metamorphic(v) => v.describe(),
            &StoneTypes::Soil(v) => v.describe(),
        }
    }
}

impl DrawChar for StoneTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &StoneTypes::Sedimentary(ref s) => s.draw_char(root, pos),
            &StoneTypes::Metamorphic(ref s) => s.draw_char(root, pos),
            &StoneTypes::Igneous(ref s) => s.draw_char(root, pos),
            &StoneTypes::Soil(ref s) => s.draw_char(root, pos),
        }
    }
}

/////// WATER
// This is a DF-type game, so... extra fidelty!
#[derive(Debug, Copy, Clone)]
pub enum LiquidPurity {
    // Helps with healing
    Pure,
    Clear,
    // Normal
    Clean,
    // Might cause health issues
    Sandy,
    Dirty,
    Murky,
    // Will kill eventually
    Muddy,
    Toxic,
}

/////// VEGITATION
// Vegiatation type, least to most rare, common to least common.
#[derive(Debug, Clone, Copy)]
pub enum VegTypes {
    // Small grasses (height 1)
    Bluegrass,
    Bentgrass,
    Ryegrass,
    // Small eadable plants (height 1-3)
    Dandelion,
    Chickweed,
    // Bushes (height 4-6)
    BroomShrub,
    Rhododendron,
    // Small trees (height 6-9)
    Crabapple,
    Redbud,
    Treetrunk,
    // Tall trees (height 10-20)
    Pine,
    Redwood,
    Banyon,
}

impl Describe for VegTypes {
    fn describe(&self) -> String {
        match self {
            &VegTypes::Bluegrass => "Bluegrass".to_string(),
            &VegTypes::Bentgrass => "Bentgrass".to_string(),
            &VegTypes::Ryegrass => "Ryegrass".to_string(),
            &VegTypes::Dandelion => "Dandelion".to_string(),
            &VegTypes::Chickweed => "Chickweed".to_string(),
            &VegTypes::BroomShrub => "Broom Shrub".to_string(),
            &VegTypes::Rhododendron => "Rhododendron".to_string(),
            &VegTypes::Crabapple => "Crabapple".to_string(),
            &VegTypes::Redbud => "Redbud".to_string(),
            &VegTypes::Pine => "Pine".to_string(),
            &VegTypes::Redwood => "Redwood".to_string(),
            &VegTypes::Banyon => "Banyon".to_string(),
            &VegTypes::Treetrunk => "Tree trunk".to_string(),
        }
    }
}

impl DrawChar for VegTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &VegTypes::Bluegrass => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '"',
                                 Color::new(0, 50, 200),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Treetrunk => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 'O',
                                 Color::new(139, 69, 19),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Bentgrass => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 ',',
                                 Color::new(0, 255, 0),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Ryegrass => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '`',
                                 Color::new(150, 200, 0),
                                 Color::new(50, 200, 50));
            }

            &VegTypes::Dandelion => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::EXCLAM_DOUBLE,
                                 Color::new(255, 255, 255),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Chickweed => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(30, 190, 30));
            }

            &VegTypes::BroomShrub => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{f4}',
                                 Color::new(79, 121, 66),
                                 Color::new(227, 255, 0));
            }
            &VegTypes::Rhododendron => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{f4}',
                                 Color::new(176, 90, 100),
                                 Color::new(227, 255, 0));
            }

            &VegTypes::Crabapple => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::CLUB,
                                 Color::new(186, 85, 211),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Redbud => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::CLUB,
                                 Color::new(216, 191, 216),
                                 Color::new(50, 200, 50));
            }

            &VegTypes::Pine => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::ARROW_N,
                                 Color::new(255, 255, 250),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Redwood => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{17}',
                                 Color::new(255, 100, 100),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Banyon => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::CLUB,
                                 Color::new(255, 255, 255),
                                 Color::new(50, 200, 50));
            }
        }
    }
}

/////// BIOME

type Ferenheight = i32;
type Percent = usize;
pub struct Biome {
    temperature_high_f: Ferenheight,
    temperature_low_f: Ferenheight,
    name: String,
    humidity_pcnt: Percent,
    percipitation_chance: Percent,
}

/////// GENERAL
// State: the 3 physical forms + fire because it's convenient.
#[derive(Debug, Copy, Clone)]
pub enum State {
    Liquid,
    Solid,
    Gas,
}

// Descriptive alias (hey, I'm a haskell programmer).
pub type Height = i32;
pub type Depth = i32;

// North is up, South is down, East is left, West is right.
#[derive(Debug, Copy, Clone)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
pub enum Slope {
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum RestrictedTile {
    Stone(StoneTypes, State),
    Vegitation(VegTypes, Height, State),
}

impl Describe for RestrictedTile {
    fn describe(&self) -> String {
        match self {
            &RestrictedTile::Stone(ref s, ref state) => {
                match state {
                    &State::Solid => format!("Rough {}", s.describe()),
                    &State::Liquid => format!("Molten {}", s.describe()),
                    _ => panic!("Panic! In the Code"),
                }
            }
            &RestrictedTile::Vegitation(veg, ..) => veg.describe(),
        }
    }
}

impl DrawChar for RestrictedTile {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &RestrictedTile::Stone(ref s, State::Solid) => {
                s.draw_char(root, pos)
            }
            &RestrictedTile::Stone(_, State::Liquid) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '~',
                                 Color::new(0, 0, 0),
                                 Color::new(255, 0, 0));
            }   
            &RestrictedTile::Stone(_, State::Gas) => {
                panic!("Stones can't be a gas!")
            }
            &RestrictedTile::Vegitation(ref v, ..) => {
                v.draw_char(root, pos)
            }
        }
    }
}

// General types of tiles (very broad) and their current state.
// FIXME: use restricted tile instead of duplicating functionality. Composition over inheritence!
#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Ramp(RestrictedTile, Slope),
    Moveable(RestrictedTile),
    Water(LiquidPurity, State, Depth),
    Stone(StoneTypes, State),
    Vegitation(VegTypes, Height, State),
    Fire,
}

impl Describe for Tile {
    fn describe(&self) -> String {
        match self {
            &Tile::Empty => "Emtpy space".to_string(),
            &Tile::Ramp(ref s, ref slope) => {
                match slope {
                    &Slope::Up => format!("Up hill of {}", s.describe()),
                    &Slope::Down => {
                        format!("Down hill of {}", s.describe())
                    }
                    &Slope::None => format!("{} floor", s.describe()),
                }
            }
            &Tile::Moveable(ref s) => {
                format!("Loose pile of {}", s.describe())
            }
            &Tile::Water(ref purity, ref state, _) => {
                let purity_str = match purity {
                    &LiquidPurity::Clean => "clean",
                    &LiquidPurity::Clear => "clear",
                    &LiquidPurity::Dirty => "dirty",
                    &LiquidPurity::Muddy => "muddy",
                    &LiquidPurity::Murky => "murky",
                    &LiquidPurity::Pure => "pure",
                    &LiquidPurity::Sandy => "sandy",
                    &LiquidPurity::Toxic => "toxic",
                };
                match state {
                    &State::Gas => format!("Cloud of {} steam", purity_str),
                    &State::Solid => format!("{} ice", purity_str),
                    &State::Liquid => format!("{} water", purity_str),
                }
            }
            &Tile::Stone(ref s, ref state) => {
                match state {
                    &State::Solid => format!("Rough {}", s.describe()),
                    &State::Liquid => format!("Molten {}", s.describe()),
                    _ => panic!("Panic! In the Code"),
                }
            }
            &Tile::Fire => "Flames".to_string(),
            &Tile::Vegitation(veg, ..) => veg.describe(),
        }
    }
}

impl DrawChar for Tile {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &Tile::Ramp(ref undertile, ref s) => {
                match s {
                    &Slope::Up => {
                        undertile.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      chars::ARROW2_N,
                                      BackgroundFlag::None);
                    }
                    &Slope::None => {
                        undertile.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      '.',
                                      BackgroundFlag::None);
                    }
                    &Slope::Down => {
                        undertile.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      chars::ARROW2_S,
                                      BackgroundFlag::None);
                    }
                }
            }
            &Tile::Moveable(ref t) => {
                match t {
                    &RestrictedTile::Stone(ref s, State::Solid) => {
                        s.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      chars::BULLET,
                                      BackgroundFlag::Set);
                    }
                    &RestrictedTile::Vegitation(ref v, ..) => {
                        match v {
                            &VegTypes::Pine |
                            &VegTypes::Banyon |
                            &VegTypes::Redwood |
                            &VegTypes::Redbud => {
                                root.put_char(pos.0 as i32,
                                              pos.1 as i32,
                                              chars::RADIO_SET,
                                              BackgroundFlag::Set);
                            }
                            f => f.draw_char(root, pos),
                        }
                    }
                    _ => panic!("Shouldn't be moveable!"),
                }
            }
            &Tile::Stone(ref s, State::Solid) => s.draw_char(root, pos),
            &Tile::Stone(_, State::Liquid) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '~',
                                 Color::new(0, 0, 0),
                                 Color::new(255, 0, 0));
            }
            &Tile::Stone(_, State::Gas) => panic!("Stones can't be a gas!"),
            &Tile::Water(_, State::Solid, _) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '#',
                                 Color::new(255, 255, 255),
                                 Color::new(100, 255, 100));
            }
            &Tile::Water(_, State::Liquid, ref depth) => {
                if depth <= &2 {
                    root.put_char_ex(pos.0 as i32,
                                     pos.1 as i32,
                                     '\u{f7}',
                                     Color::new(0, 105, 148),
                                     Color::new(0, 159, 225));
                } else {
                    root.put_char_ex(pos.0 as i32,
                                     pos.1 as i32,
                                     format!("{}", depth)
                                         .bytes()
                                         .take(1)
                                         .collect::<Vec<_>>()
                                         [0] as
                                         char,
                                     Color::new(0, 105, 148),
                                     Color::new(0, 159, 225));
                }
            }   
            &Tile::Water(_, State::Gas, _) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{a7}',
                                 Color::new(0, 105, 148),
                                 Color::new(0, 159, 225));
            }
            &Tile::Vegitation(ref v, ..) => v.draw_char(root, pos),
            &Tile::Fire => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::YEN,
                                 Color::new(227, 140, 45),
                                 Color::new(255, 0, 0));
            }
            &Tile::Empty => {}
        }
    }
}

// A unit is a 1x1 cross section of the layered world, including a ref
// to the biome its part of.
// Each tile is 1 foot tall.
pub struct Unit {
    pub tiles: Vec<Tile>,
    pub biomes: Vec<Biome>,
}

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

fn add_hill(heightmap: *mut tcod_sys::TCOD_heightmap_t,
            (hmw, hmh): (usize, usize),
            num_hills: i32,
            base_radius: f32,
            radius: f32,
            height: f32,
            rndn: tcod_sys::TCOD_random_t) {
    unsafe {
        for i in 0..num_hills {
            let radius =
                tcod_sys::TCOD_random_get_float(rndn,
                                                base_radius *
                                                    (1.0 - radius),
                                                base_radius *
                                                    (1.0 + radius));
            let xh = tcod_sys::TCOD_random_get_int(rndn, 0, hmw as i32 - 1);
            let yh = tcod_sys::TCOD_random_get_int(rndn, 0, hmh as i32 - 1);
            tcod_sys::TCOD_heightmap_add_hill(heightmap,
                                              xh as f32,
                                              yh as f32,
                                              radius,
                                              height);
        }
    }
}

const THRESHOLD: f32 = 0.5;
const SEA_LEVEL: f32 = 23.0;
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
            .random(random::Rng::new_with_seed(random::Algo::MT, seed))
            .init();

        let (sx, sy) = size;

        // And here we see a small C hiding in the maw of a Rust. <low-level>
        let heightmap =
            unsafe { tcod_sys::TCOD_heightmap_new(sx as i32, sy as i32) };
        unsafe {
            let rndn =
                tcod_sys::TCOD_random_new_from_seed(tcod_sys::TCOD_RNG_MT,
                                                    seed);
            let noise = tcod_sys::TCOD_noise_new(2, 0.7, 0.1, rndn);
            tcod_sys::TCOD_noise_set_type(noise,
                                          tcod_sys::TCOD_NOISE_PERLIN);
            tcod_sys::TCOD_heightmap_add_fbm(heightmap,
                                             noise,
                                             2.20 * (sx as f32) / 400.0,
                                             2.20 * (sx as f32) / 400.0,
                                             0.0,
                                             0.0,
                                             10.0,
                                             1.0,
                                             2.05);
            tcod_sys::TCOD_heightmap_normalize(heightmap, 0.0, 100.0);
            add_hill(heightmap,
                     size,
                     600,
                     16.0 * sx as f32 / 200.0,
                     0.7,
                     0.3,
                     rndn);
            tcod_sys::TCOD_heightmap_normalize(heightmap, 0.0, 100.0);
            tcod_sys::TCOD_heightmap_rain_erosion(heightmap,
                                                  (sx * sy) as i32,
                                                  0.03,
                                                  0.01,
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
                .random(random::Rng::new_with_seed(random::Algo::MT, seed))
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
                    let dist = (SEA_LEVEL - height) as isize;
                    for z in 0..dist {
                        tiles.push(Tile::Water(World::purity(),
                                               State::Liquid,
                                               (dist - z) as i32));
                    }
                } else {
                    for z in 0..(height as isize - 1) {
                        biomes.push(world.biome_from_height(z));
                        tiles.push(world.rock_type((x, y), z));
                    }
                    if height <= VEG_THRESHOLD {
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
                            if slope < -RAMP_THRESHOLD {
                                tiles[last] =
                            Tile::Ramp(restricted_from_tile(tile.clone()),
                                       Slope::Down);
                            } else if slope > RAMP_THRESHOLD {
                                tiles[last] =
                            Tile::Ramp(restricted_from_tile(tile.clone()),
                                       Slope::Up);
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
                                           y as i32) * THRESHOLD
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
            temperature_high_f: 75,
            temperature_low_f: 52,
            name: "LowMed".to_string(),
            humidity_pcnt: 60,
            percipitation_chance: 70,
        }
    }

    fn rock_choice<T: Clone>(list: &[T; 2], rn: isize) -> T {
        println!("{}", rn);
        if rn < 0 {
            list[0].clone()
        } else {
            list[1].clone()
        }
    }
    pub fn rock_type(&self, (x, y): (usize, usize), height: isize) -> Tile {
        let rn =
            (self.stone_vein_noise
                 .get_fbm(&mut [x as f32, y as f32, height as f32], 6) *
                 10.0) as isize;
        let sedimentary = &[SedimentaryRocks::Conglomerate,
                            SedimentaryRocks::Limestone];
        let igneous = &[IgneousRocks::Obsidian, IgneousRocks::Basalt];
        let metamorphic = &[MetamorphicRocks::Marble,
                            MetamorphicRocks::Gneiss];
        Tile::Stone(// Stone type
                    if height < 10 {
                        let v = World::rock_choice(igneous, rn);
                        StoneTypes::Igneous(v.clone())
                    } else if height <= 20 {
                        let v = World::rock_choice(sedimentary, rn);
                        StoneTypes::Sedimentary(v.clone())
                    } else {
                        let v = World::rock_choice(metamorphic, rn);
                        StoneTypes::Metamorphic(v.clone())
                    },

                    // State
                    if height < 3 {
                        if rn < 30 { State::Liquid } else { State::Solid }
                    } else {
                        State::Solid
                    })
    }

    pub fn get_vegetation(&self, (x, y): (usize, usize)) -> Tile {
        let vn = self.vegetation_noise
                     .get_fbm(&mut [x as f32, y as f32], 1) *
            100.0;
        let veg_levels =
            vec![[VegTypes::Bluegrass,
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
                 [VegTypes::Pine, VegTypes::Crabapple, VegTypes::Pine],
                 [VegTypes::Redwood, VegTypes::Pine, VegTypes::Banyon]];
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
    pub screen: (i32, i32),
    pub cursor: (i32, i32),
    pub level: i32,
    pub life: Vec<Box<life::Living>>,
    pub map: World,
    pub highest_level: usize,
    pub tcod_map: map::Map,
}

impl WorldState {
    pub fn new(world: World, s: (usize, usize)) -> WorldState {
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
        WorldState {
            screen: (0, 0),
            level: 31,
            highest_level: toplevel.unwrap(),
            cursor: (0, 0),
            life: vec![],
            map: world,
            tcod_map: map::Map::new(s.0 as i32, s.1 as i32),
        }
    }
}
