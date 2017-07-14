extern crate tcod;

use std;

use draw::{Describe, DrawChar};
use tcod::colors::Color;

use tcod::RootConsole;
use tcod::chars;
use tcod::console::{BackgroundFlag, Console};

pub const TILES: bool = true;
pub const BASE: u32 = 256;
const TILES_BENTGRASS: u32 = BASE;
const TILES_BLUEGRASS: u32 = (BASE + 1);
const TILES_CHICKWEED: u32 = (BASE + 2);
const TILES_DANDELION: u32 = (BASE + 3);
const TILES_PASTUREGRASS: u32 = (BASE + 4);
const TILES_RYEGRASS: u32 = (BASE + 5);
const TILES_CLAY: u32 = (BASE + 6);
const TILES_CONGLOMERATE: u32 = (BASE + 7);
const TILES_GNEISS: u32 = (BASE + 8);
const TILES_LIMESTONE: u32 = (BASE + 9);
const TILES_LOAMY: u32 = (BASE + 10);
const TILES_MARBLE: u32 = (BASE + 11);
const TILES_OBSIDIAN: u32 = (BASE + 12);
const TILES_PEATY: u32 = (BASE + 13);
const TILES_SANDY: u32 = (BASE + 14);
const TILES_SILTY: u32 = (BASE + 15);
const TILES_WATER: u32 = (BASE + 16);
const TILES_LAVA: u32 = (BASE + 17);
const TILES_CRABAPPLE: u32 = (BASE + 18);
const TILES_BASALT: u32 = (BASE + 19);
const TILES_BANYON: u32 = (BASE + 20);
const TILES_BROOMSHRUB: u32 = (BASE + 21);
const TILES_PINE: u32 = (BASE + 22);
const TILES_REDBUD: u32 = (BASE + 23);
const TILES_REDWOOD: u32 = (BASE + 24);
const TILES_RHODODENDRON: u32 = (BASE + 25);
const TILES_TREETRUNK: u32 = (BASE + 26);

/////// ROCK
// Possible igneous rock kinds
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IgneousRocks {
    Obsidian,
    Basalt,
}

impl Describe for IgneousRocks {
    fn describe(&self) -> String {
        match self {
            &IgneousRocks::Obsidian => {
                "Igneous obisidian".to_string()
            }
            &IgneousRocks::Basalt => "Igneous basalt".to_string(),
        }
    }
}

impl DrawChar for IgneousRocks {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &IgneousRocks::Obsidian => {
                let chr = if TILES {
                    std::char::from_u32(TILES_OBSIDIAN)
                        .unwrap()
                } else {
                    chars::BLOCK1
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(12, 12, 12)
                                 },
                                 Color::new(2, 2, 2))
            }
            &IgneousRocks::Basalt => {
                let chr = if TILES {
                    std::char::from_u32(TILES_BASALT)
                        .unwrap()
                } else {
                    chars::BLOCK3
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(40, 40, 12)
                                 },
                                 Color::new(2, 2, 2))
            }
        };
    }
}

// Possible metamorphic rock kinds
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MetamorphicRocks {
    Gneiss,
    Marble,
}

impl Describe for MetamorphicRocks {
    fn describe(&self) -> String {
        match self {
            &MetamorphicRocks::Gneiss => {
                "Metamorphic gneiss".to_string()
            }
            &MetamorphicRocks::Marble => {
                "Metamorphic marble".to_string()
            }
        }
    }
}

impl DrawChar for MetamorphicRocks {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &MetamorphicRocks::Gneiss => {
                let chr = if TILES {
                    std::char::from_u32(TILES_GNEISS)
                        .unwrap()
                } else {
                    chars::BLOCK2
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(125, 85, 62)
                                 },
                                 Color::new(2, 2, 2))
            }
            &MetamorphicRocks::Marble => {
                let chr = if TILES {
                    std::char::from_u32(TILES_MARBLE)
                        .unwrap()
                } else {
                    chars::BLOCK1
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(250, 250, 250)
                                 },
                                 Color::new(12, 12, 12))
            }
        };
    }
}

// Possible Sedimentary rock kinds
#[derive(Debug, Copy, Clone, PartialEq)]
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
                let chr = if TILES {
                    std::char::from_u32(TILES_LIMESTONE)
                        .unwrap()
                } else {
                    chars::BLOCK3
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(125, 85, 62)
                                 },
                                 Color::new(255, 255, 255))
            }
            &SedimentaryRocks::Conglomerate => {
                let chr = if TILES {
                    std::char::from_u32(TILES_CONGLOMERATE)
                        .unwrap()
                } else {
                    chars::BLOCK2
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(130, 100, 50)
                                 },
                                 Color::new(200, 200, 200))
            }
        };
    }
}

// Soil types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SoilTypes {
    Clay,
    Sandy,
    Silty,
    Peaty,
    Loamy,
}

impl Describe for SoilTypes {
    fn describe(&self) -> String {
        match self {
            &SoilTypes::Clay => "Clay soil".to_string(),
            &SoilTypes::Sandy => "Sandy soil".to_string(),
            &SoilTypes::Silty => "Silty soil".to_string(),
            &SoilTypes::Peaty => "Peaty soil".to_string(),
            &SoilTypes::Loamy => "Loamy soil".to_string(),
        }
    }
}

impl DrawChar for SoilTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &SoilTypes::Clay => {
                let chr = if TILES {
                    std::char::from_u32(TILES_CLAY).unwrap()
                } else {
                    '='
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(167, 107, 41)
                                 },
                                 Color::new(191, 100, 35))
            }
            &SoilTypes::Sandy => {
                let chr = if TILES {
                    std::char::from_u32(TILES_SANDY)
                        .unwrap()
                } else {
                    '='
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(167, 107, 41)
                                 },
                                 Color::new(191, 100, 35))
            }
            &SoilTypes::Silty => {
                let chr = if TILES {
                    std::char::from_u32(TILES_SILTY)
                        .unwrap()
                } else {
                    '='
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(133, 126, 108)
                                 },
                                 Color::new(99, 94, 80))
            }
            &SoilTypes::Peaty => {
                let chr = if TILES {
                    std::char::from_u32(TILES_PEATY)
                        .unwrap()
                } else {
                    '='
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(159, 145, 95)
                                 },
                                 Color::new(119, 108, 71))
            }
            &SoilTypes::Loamy => {
                let chr = if TILES {
                    std::char::from_u32(TILES_LOAMY)
                        .unwrap()
                } else {
                    '='
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(86, 59, 56)
                                 },
                                 Color::new(64, 44, 41))
            }   
        }
    }
}

// Stone types (SCIENCE!)
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
                let chr = if TILES {
                    std::char::from_u32(TILES_PASTUREGRASS)
                        .unwrap()
                } else {
                    '"'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 50, 200)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Treetrunk => {
                let chr = if TILES {
                    std::char::from_u32(TILES_TREETRUNK)
                        .unwrap()
                } else {
                    'O'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(139, 69, 19)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Bentgrass => {
                let chr = if TILES {
                    std::char::from_u32(TILES_BENTGRASS)
                        .unwrap()
                } else {
                    ','
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 255, 0)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Ryegrass => {
                let chr = if TILES {
                    std::char::from_u32(TILES_RYEGRASS)
                        .unwrap()
                } else {
                    '`'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(150, 200, 0)
                                 },
                                 Color::new(50, 200, 50));
            }

            &VegTypes::Dandelion => {
                let chr = if TILES {
                    std::char::from_u32(TILES_DANDELION)
                        .unwrap()
                } else {
                    chars::EXCLAM_DOUBLE
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 255, 255)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Chickweed => {
                let chr = if TILES {
                    std::char::from_u32(TILES_CHICKWEED)
                        .unwrap()
                } else {
                    '!'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 255, 254)
                                 },
                                 Color::new(30, 190, 30));
            }

            &VegTypes::BroomShrub => {
                let chr = if TILES {
                    std::char::from_u32(TILES_BROOMSHRUB)
                        .unwrap()
                } else {
                    '\u{f4}'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(79, 121, 66)
                                 },
                                 Color::new(227, 255, 0));
            }
            &VegTypes::Rhododendron => {
                let chr = if TILES {
                    std::char::from_u32(TILES_RHODODENDRON)
                        .unwrap()
                } else {
                    '\u{f4}'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(176, 90, 100)
                                 },
                                 Color::new(227, 255, 0));
            }

            &VegTypes::Crabapple => {
                let chr = if TILES {
                    std::char::from_u32(TILES_CRABAPPLE)
                        .unwrap()
                } else {
                    chars::CLUB
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(186, 85, 211)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Redbud => {
                let chr = if TILES {
                    std::char::from_u32(TILES_REDBUD)
                        .unwrap()
                } else {
                    chars::CLUB
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(216, 191, 216)
                                 },
                                 Color::new(50, 200, 50));
            }

            &VegTypes::Pine => {
                let chr = if TILES {
                    std::char::from_u32(TILES_PINE).unwrap()
                } else {
                    chars::ARROW_N
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 255, 250)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Redwood => {
                let chr = if TILES {
                    std::char::from_u32(TILES_REDWOOD)
                        .unwrap()
                } else {
                    '\u{17}'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 100, 100)
                                 },
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Banyon => {
                let chr = if TILES {
                    std::char::from_u32(TILES_BANYON)
                        .unwrap()
                } else {
                    chars::CLUB
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 255, 255)
                                 },
                                 Color::new(50, 200, 50));
            }
        }
    }
}

/////// BIOME

type Ferenheight = i32;
type Percent = usize;
#[derive(Copy, Clone, PartialEq)]
pub enum BiomeType {
    Swamp,
    Jungle,
    Forest,
    Pasture,
    Beach,
}
#[derive(Copy, Clone, PartialEq)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub temperature_night_f: Ferenheight,
    pub temperature_day_f: Ferenheight,
    pub percipitation_chance: Percent,
}

/////// GENERAL
// State: the 3 physical forms + fire because it's convenient.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Liquid,
    Solid,
    Gas,
}

// Descriptive alias (hey, I'm a haskell programmer).
pub type Height = i32;
pub type Depth = i32;

// North is up, South is down, East is left, West is right.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Slope {
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RestrictedTile {
    Stone(StoneTypes, State),
    Vegitation(VegTypes, Height, State),
}

impl Describe for RestrictedTile {
    fn describe(&self) -> String {
        match self {
            &RestrictedTile::Stone(ref s, ref state) => {
                match state {
                    &State::Solid => {
                        format!("Rough {}", s.describe())
                    }
                    &State::Liquid => {
                        format!("Molten {}", s.describe())
                    }
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
                let chr = if TILES {
                    std::char::from_u32(TILES_LAVA).unwrap()
                } else {
                    '~'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 0, 0)
                                 },
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
#[derive(Debug, Copy, Clone, PartialEq)]
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
                    &Slope::Up => {
                        format!("Up hill of {}", s.describe())
                    }
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
                    &State::Gas => {
                        format!("Cloud of {} steam", purity_str)
                    }
                    &State::Solid => format!("{} ice", purity_str),
                    &State::Liquid => format!("{} water", purity_str),
                }
            }
            &Tile::Stone(ref s, ref state) => {
                match state {
                    &State::Solid => {
                        format!("Rough {}", s.describe())
                    }
                    &State::Liquid => {
                        format!("Molten {}", s.describe())
                    }
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
                        if !TILES {
                            root.put_char(pos.0 as i32,
                                          pos.1 as i32,
                                          chars::ARROW2_N,
                                          BackgroundFlag::None);
                        }
                    }
                    &Slope::None => {
                        undertile.draw_char(root, pos);
                        if !TILES {
                            root.put_char(pos.0 as i32,
                                          pos.1 as i32,
                                          '.',
                                          BackgroundFlag::None);
                        }
                    }
                    &Slope::Down => {
                        undertile.draw_char(root, pos);
                        if !TILES {
                            root.put_char(pos.0 as i32,
                                          pos.1 as i32,
                                          chars::ARROW2_S,
                                          BackgroundFlag::None);
                        }
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
            &Tile::Stone(ref s, State::Solid) => {
                s.draw_char(root, pos)
            }
            &Tile::Stone(_, State::Liquid) => {
                let chr = if TILES {
                    std::char::from_u32(TILES_LAVA).unwrap()
                } else {
                    '~'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 0, 0)
                                 },
                                 Color::new(255, 0, 0));
            }
            &Tile::Stone(_, State::Gas) => {
                panic!("Stones can't be a gas!")
            }
            &Tile::Water(_, State::Solid, _) => {
                let chr = if TILES {
                    std::char::from_u32(TILES_OBSIDIAN)
                        .unwrap()
                } else {
                    chars::BLOCK1
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '#',
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(255, 255, 255)
                                 },
                                 Color::new(100, 255, 100));
            }
            &Tile::Water(_, State::Liquid, ref depth) => {
                let chr = if TILES {
                    std::char::from_u32(TILES_WATER)
                        .unwrap()
                } else {
                    '\u{f7}'
                };
                if depth <= &2 {
                    root.put_char_ex(pos.0 as i32,
                                     pos.1 as i32,
                                     chr,
                                     if TILES {
                                         Color::new(255, 255, 255)
                                     } else {
                                         Color::new(0, 105, 148)
                                     },
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
                                     if TILES {
                                         Color::new(255, 255, 255)
                                     } else {
                                         Color::new(0, 105, 148)
                                     },
                                     Color::new(0, 159, 225));
                }
            }   
            &Tile::Water(_, State::Gas, _) => {
                let chr = if TILES {
                    std::char::from_u32(TILES_OBSIDIAN)
                        .unwrap()
                } else {
                    chars::BLOCK1
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{a7}',
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 105, 148)
                                 },
                                 Color::new(0, 159, 225));
            }
            &Tile::Vegitation(ref v, ..) => v.draw_char(root, pos),
            &Tile::Fire => {
                let chr = if TILES {
                    std::char::from_u32(TILES_OBSIDIAN)
                        .unwrap()
                } else {
                    chars::BLOCK1
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::YEN,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(227, 140, 45)
                                 },
                                 Color::new(255, 0, 0));
            }
            &Tile::Empty => {}
        }
    }
}
