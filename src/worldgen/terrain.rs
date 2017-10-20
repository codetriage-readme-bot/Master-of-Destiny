extern crate tcod;
extern crate rand;

use std;

use draw::{Describe, DrawChar, FramedDraw};
use life::animal::Species;
use physics::PhysicsActor;
use tcod::colors::Color;
use time::Calendar;
use utils::Point2D;
use worldgen::Frames;

use tcod::RootConsole;
use tcod::chars;
use tcod::console::{BackgroundFlag, Console};

pub const TILES: bool = true;
pub const BASE: isize = 256;
pub enum Tiles {
    Bentgrass = BASE,
    Bluegrass,
    Chickweed,
    Dandelion,
    Pasturegrass,
    Ryegrass,
    Clay,
    Conglomerate,
    Gneiss,
    Limestone,
    Loamy,
    Marble,
    Obsidian,
    Peaty,
    Sandy,
    Silty,
    Water,
    Lava,
    Crabapple,
    Basalt,
    Banyon,
    Broomshrub,
    Pine,
    Redbud,
    Redwood,
    Rhododendron,
    Treetrunk,
    Puddle,
    Ice,
    Snow,
    Woodwall,
    Stonewall,
}

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
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &IgneousRocks::Obsidian => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Obsidian as u32)
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
                    std::char::from_u32(Tiles::Basalt as u32)
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
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &MetamorphicRocks::Gneiss => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Gneiss as u32)
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
                    std::char::from_u32(Tiles::Marble as u32)
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
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &SedimentaryRocks::Limestone => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Limestone as u32)
                        .unwrap()
                } else {
                    chars::BLOCK3
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(150, 150, 150)
                                 } else {
                                     Color::new(125, 85, 62)
                                 },
                                 Color::new(255, 255, 255))
            }
            &SedimentaryRocks::Conglomerate => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Conglomerate as u32)
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
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &SoilTypes::Clay => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Clay as u32)
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
            &SoilTypes::Sandy => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Sandy as u32)
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
                    std::char::from_u32(Tiles::Silty as u32)
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
                    std::char::from_u32(Tiles::Peaty as u32)
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
                    std::char::from_u32(Tiles::Loamy as u32)
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
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
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
pub enum VegType {
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

impl Describe for VegType {
    fn describe(&self) -> String {
        match self {
            &VegType::Bluegrass => "Bluegrass".to_string(),
            &VegType::Bentgrass => "Bentgrass".to_string(),
            &VegType::Ryegrass => "Ryegrass".to_string(),
            &VegType::Dandelion => "Dandelion".to_string(),
            &VegType::Chickweed => "Chickweed".to_string(),
            &VegType::BroomShrub => "Broom Shrub".to_string(),
            &VegType::Rhododendron => "Rhododendron".to_string(),
            &VegType::Crabapple => "Crabapple".to_string(),
            &VegType::Redbud => "Redbud".to_string(),
            &VegType::Pine => "Pine".to_string(),
            &VegType::Redwood => "Redwood".to_string(),
            &VegType::Banyon => "Banyon".to_string(),
            &VegType::Treetrunk => "Tree trunk".to_string(),
        }
    }
}

impl DrawChar for VegType {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &VegType::Bluegrass => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Pasturegrass as u32)
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
            &VegType::Treetrunk => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Treetrunk as u32)
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
            &VegType::Bentgrass => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Bentgrass as u32)
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
            &VegType::Ryegrass => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Ryegrass as u32)
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

            &VegType::Dandelion => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Dandelion as u32)
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
            &VegType::Chickweed => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Chickweed as u32)
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

            &VegType::BroomShrub => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Broomshrub as u32)
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
            &VegType::Rhododendron => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Rhododendron as u32)
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

            &VegType::Crabapple => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Crabapple as u32)
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
            &VegType::Redbud => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Redbud as u32)
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

            &VegType::Pine => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Pine as u32)
                        .unwrap()
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
            &VegType::Redwood => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Redwood as u32)
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
            &VegType::Banyon => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Banyon as u32)
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

type Ferenheight = f32;
type Percent = f32;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BiomeType {
    Swamp,
    Jungle,
    Forest,
    Desert,
    Pasture,
    Beach,
    Water,
}

impl BiomeType {
    pub fn stringified(&self) -> String {
        use self::BiomeType::*;
        match self {
            &Swamp => "s",
            &Jungle => "j",
            &Forest => "f",
            &Desert => "d",
            &Pasture => "p",
            &Beach => "b",
            &Water => "w",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub temperature_night_f: Ferenheight,
    pub temperature_day_f: Ferenheight,
    pub percipitation_chance: Percent,
}

pub const WATER_BIOME: Biome = Biome {
    biome_type: BiomeType::Water,
    temperature_night_f: -5.0,
    temperature_day_f: 70.0,
    percipitation_chance: 80.0,
};

impl Biome {
    pub fn survives(&self, veg: VegType) -> bool {
        use self::VegType::*;
        if self.biome_type == BiomeType::Desert {
            return false;
        }
        match veg {
            Bluegrass => {
                self.biome_type == BiomeType::Forest ||
                    self.biome_type == BiomeType::Pasture ||
                    self.biome_type == BiomeType::Swamp
            }
            Bentgrass => {
                self.biome_type == BiomeType::Forest ||
                    self.biome_type == BiomeType::Pasture
            }
            Ryegrass => self.biome_type == BiomeType::Pasture,
            Dandelion => self.biome_type != BiomeType::Swamp,
            Chickweed => {
                self.biome_type != BiomeType::Swamp &&
                    self.biome_type != BiomeType::Jungle
            }
            BroomShrub => {
                self.biome_type == BiomeType::Forest ||
                    self.biome_type == BiomeType::Pasture
            }
            Crabapple => {
                self.biome_type == BiomeType::Forest ||
                    self.biome_type == BiomeType::Beach ||
                    self.biome_type == BiomeType::Pasture
            }
            Rhododendron => self.biome_type == BiomeType::Jungle,
            Redbud => self.biome_type == BiomeType::Pasture,
            Pine => {
                self.biome_type == BiomeType::Forest ||
                    self.biome_type == BiomeType::Pasture
            }
            Redwood => self.biome_type == BiomeType::Forest,
            Banyon => self.biome_type == BiomeType::Jungle,
            _ => false,
        }
    }
}

/////// GENERAL
// State: the 3 physical forms + fire because it's convenient.
/// Physical state of an object, based on chemistry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Liquid,
    Solid,
    Gas,
}

/// Descriptive alias (hey, I'm a haskell programmer).
pub type Height = i32;
pub type Depth = i32;

/// North is up, South is down, East is left, West is right.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

/// Tile types that can be defined to be moveable or as ramps.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RestrictedTile {
    Stone(StoneTypes, State),
    Vegetation(VegType, Height, State),
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
                    _ => unreachable!(),
                }
            }
            &RestrictedTile::Vegetation(veg, ..) => veg.describe(),
        }
    }
}

impl DrawChar for RestrictedTile {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &RestrictedTile::Stone(ref s, State::Solid) => {
                s.draw_char(root, pos)
            }
            &RestrictedTile::Stone(_, State::Liquid) => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Lava as u32)
                        .unwrap()
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
            &RestrictedTile::Stone(_, State::Gas) => unreachable!(),
            &RestrictedTile::Vegetation(ref v, ..) => {
                v.draw_char(root, pos)
            }
        }
    }
}

/// The
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tool {
    // Weapons
    Sword,
    Spear,

    // Armor
    Sheild,
    Breastplate,
    Leggings,
    Helmet,
    Boots,
    Shoes,

    // Tools
    Hammer,
    Pickaxe,
    Net,
    FishingPole,
    Fork,
    Knife,
    Spoon,
    Bowl,
    Goblet,
    Cup,
    Plate,
    Wheel,
}

impl DrawChar for Tool {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {}
}

type Quantity = u8;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Food {
    Meat(Species),
    Herb(VegType),
    Water(Quantity),
}

impl DrawChar for Food {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &Food::Meat(_) => {
                root.set_default_foreground(Color::new(138, 7, 7));
                root.put_char(pos.0 as i32,
                              pos.1 as i32,
                              '%',
                              BackgroundFlag::None);
            }
            &Food::Herb(_) => {
                root.set_default_foreground(Color::new(139, 172, 44));
                root.put_char(pos.0 as i32,
                              pos.1 as i32,
                              '#',
                              BackgroundFlag::None);
            }
            _ => {}
        }
        root.set_default_foreground(Color::new(255, 255, 255));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    Wood(VegType),
    Stone(StoneTypes),
}

impl DrawChar for Material {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &Material::Wood(_) => {
                root.set_default_foreground(Color::new(139, 69, 19));
                root.put_char(pos.0 as i32,
                              pos.1 as i32,
                              '/',
                              BackgroundFlag::None);
            }
            &Material::Stone(_) => {
                root.set_default_foreground(Color::new(139,
                                                       141,
                                                       122));
                root.put_char(pos.0 as i32,
                              pos.1 as i32,
                              '/',
                              BackgroundFlag::None);
            }
        }
        root.set_default_foreground(Color::new(255, 255, 255));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Magic {
    potency: u8,
    cursed: bool,
    dates: Option<Calendar>,
}

type Weight = u8;
type Length = u8;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Item {
    Tool(Tool, Weight, Length, Option<Magic>),
    Food(Food),
    Material(Material),
}

impl DrawChar for Item {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &Item::Tool(t, ..) => t.draw_char(root, pos),
            &Item::Food(f) => f.draw_char(root, pos),
            &Item::Material(m) => m.draw_char(root, pos),
        }
    }
}

impl Describe for Item {
    fn describe(&self) -> String {
        match self {
            &Item::Tool(tk, _w, l, m) => {
                if m.is_some() {
                    format!("an enchanted length {} {:?}", l, tk)
                } else {
                    format!("a normal length {} {:?}", l, tk)
                }
            }
            &Item::Food(f) => format!("a {:?}", f),
            &Item::Material(m) => {
                format!("some loose, piled {:?}", m)
            }
        }
    }
}

/// General types of tiles (very broad) and their current state.
///
/// FIXME: use restricted tile instead of duplication
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Ramp(RestrictedTile),
    Moveable(RestrictedTile),
    Item(Item),
    Water(LiquidPurity, State, Depth),
    Stone(StoneTypes, State),
    Vegetation(VegType, Height, State),
    Fire,
}

impl PhysicsActor for Tile {
    fn solid(&self) -> bool {
        match self {
            &Tile::Stone(..) => true,
            &Tile::Ramp(..) => true,
            _ => false,
        }
    }

    fn heavy(&self) -> bool {
        match self {
            &Tile::Stone(..) => true,
            &Tile::Moveable(..) => true,
            _ => false,
        }
    }
}

impl Describe for Tile {
    fn describe(&self) -> String {
        match self {
            &Tile::Empty => "Emtpy space".to_string(),
            &Tile::Ramp(ref s) => "A ramp".to_string(),
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
                    _ => unreachable!(),
                }
            }
            &Tile::Fire => "Flames".to_string(),
            &Tile::Vegetation(veg, ..) => veg.describe(),
            &Tile::Item(i) => i.describe(),
        }
    }
}

impl DrawChar for Tile {
    fn draw_char(&self, root: &mut RootConsole, pos: Point2D) {
        match self {
            &Tile::Ramp(_) => {}
            &Tile::Moveable(ref t) => {
                match t {
                    &RestrictedTile::Stone(ref s, State::Solid) => {
                        s.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      chars::BULLET,
                                      BackgroundFlag::Set);
                    }
                    &RestrictedTile::Vegetation(ref v, ..) => {
                        match v {
                            &VegType::Pine |
                            &VegType::Banyon |
                            &VegType::Redwood |
                            &VegType::Redbud => {
                                root.put_char(pos.0 as i32,
                                              pos.1 as i32,
                                              chars::RADIO_SET,
                                              BackgroundFlag::Set);
                            }
                            f => f.draw_char(root, pos),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            &Tile::Stone(ref s, State::Solid) => {
                s.draw_char(root, pos)
            }
            &Tile::Stone(_, State::Liquid) => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Lava as u32)
                        .unwrap()
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
            &Tile::Stone(_, State::Gas) => unreachable!(),
            &Tile::Water(_, State::Solid, _) => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Ice as u32)
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
            &Tile::Water(_, State::Liquid, _) => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Water as u32)
                        .unwrap()
                } else {
                    '\u{f7}'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 105, 148)
                                 },
                                 Color::new(0, 159, 225));
            }
            &Tile::Water(_, State::Gas, _) => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Obsidian as u32)
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
            &Tile::Vegetation(ref v, ..) => v.draw_char(root, pos),
            &Tile::Fire => {
                let chr = if TILES {
                    std::char::from_u32(Tiles::Obsidian as u32)
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
            &Tile::Empty => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 ' ',
                                 Color::new(150, 150, 150),
                                 Color::new(150, 150, 150));
            }
            &Tile::Item(i) => i.draw_char(root, pos),
        }
    }
}

impl FramedDraw for Tile {
    fn draw_framed_char(&self,
                        root: &mut RootConsole,
                        pos: Point2D,
                        time: usize,
                        frames_hash: &Frames) {
        use self::rand::Rng;
        match self {
            &Tile::Water(_, State::Liquid, _) => {
                let frames = &frames_hash["Water"];
                let chr = if TILES {
                    let rand = *rand::thread_rng()
                        .choose(&[0, 1, 2])
                        .unwrap();
                    let cframe = frames[(time + rand) % frames.len()];
                    std::char::from_u32(BASE as u32 + cframe as u32)
                        .unwrap()
                } else {
                    '\u{f7}'
                };
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chr,
                                 if TILES {
                                     Color::new(255, 255, 255)
                                 } else {
                                     Color::new(0, 105, 148)
                                 },
                                 Color::new(0, 159, 225));
            }
            _ => self.draw_char(root, pos),
        }
    }
}
