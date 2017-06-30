use life::Animal;

use tcod::RootConsole;
use tcod::chars;
use tcod::colors::Color;
use tcod::console::Console;
use tcod::noise::{Noise, NoiseType};
use tcod::random::{Algo, Rng};

trait DrawChar {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32));
}

/////// ROCK
// Possible igneous rock kinds
pub enum IgneousRocks {
    Obsidian,
    Basalt,
}

impl DrawChar for IgneousRocks {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32)) {
        match self {
            IgneousRocks::Obsidian => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK1,
                                 Color::new(12, 12, 12),
                                 Color::new(2, 2, 2))
            }
            IgneousRocks::Basalt => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK2,
                                 Color::new(20, 20, 12),
                                 Color::new(2, 2, 2))
            }
        };
    }
}

// Possible metamorphic rock kinds
pub enum MetamorphicRocks {
    Gneiss,
    Marble,
}

impl DrawChar for MetamorphicRocks {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32)) {
        match self {
            MetamorphicRocks::Gneiss => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK2,
                                 Color::new(125, 85, 62),
                                 Color::new(2, 2, 2))
            }
            MetamorphicRocks::Marble => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK1,
                                 Color::new(250, 250, 250),
                                 Color::new(12, 12, 12))
            }
        };
    }
}

// Possible Sedimentary rock kinds
pub enum SedimentaryRocks {
    Limestone,
    Conglomerate,
}

impl DrawChar for SedimentaryRocks {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32)) {
        match self {
            SedimentaryRocks::Limestone => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK3,
                                 Color::new(125, 85, 62),
                                 Color::new(255, 255, 255))
            }
            SedimentaryRocks::Conglomerate => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::BLOCK2,
                                 Color::new(130, 100, 50),
                                 Color::new(200, 200, 200))
            }
        };
    }
}

// Stone types (SCIENCE!)
pub enum StoneTypes {
    Sedimentary(SedimentaryRocks),
    Igneous(IgneousRocks),
    Metamorphic(MetamorphicRocks),
}

impl DrawChar for StoneTypes {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32)) {
        match self {
            StoneTypes::Sedimentary(s) => s.draw_char(root, pos),
            StoneTypes::Metamorphic(s) => s.draw_char(root, pos),
            StoneTypes::Igneous(s) => s.draw_char(root, pos),
        }
    }
}

/////// WATER
// This is a DF-type game, so... extra fidelty!
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
pub enum VegTypes {
    // Small grasses (height 1)
    Bluegrass,
    Bentgrass,
    Ryegrass,
    // Small eadable plants (height 1)
    Dandelion,
    Chickweed,
    Brassica,
    // Small weeds (height 1)
    Amaranth,
    Bermudagrass,
    Bindweed,
    // Small bushes (height 2)
    WoolyThyme,
    QuartzRoseVerbena,
    RocksprayCotoneaster,
    // Small trees (height 7-9)
    Crabapple,
    Redbud,
    CrapeMyrtle,
    // Medium trees (height 20-30)
    AcerRubrum,
    RedBuckeye,
    MinnesotaStrainRedbud,
    // Tall trees (height 50-60)
    BaldCypress,
    BoxElder,
    ChineseTallowTree,
}

impl DrawChar for VegTypes {
    fn draw_char(self, root: &mut RootConsole, pos: (i32, i32)) {
        match self {
            Bluegrass => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '"',
                                 Color::new(0, 200, 150),
                                 Color::new(50, 200, 50));
            }
            Bentgrass => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 ',',
                                 Color::new(0, 255, 0),
                                 Color::new(50, 200, 50));
            }
            Ryegrass => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '`',
                                 Color::new(150, 200, 0),
                                 Color::new(50, 200, 50));
            }

            Dandelion => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::EXCLAM_DOUBLE,
                                 Color::new(255, 255, 255),
                                 Color::new(50, 200, 50));
            }
            Chickweed => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(30, 190, 30));
            }
            Brassica => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '.',
                                 Color::new(152, 251, 152),
                                 Color::new(50, 250, 50));
            }

            Amaranth => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 ':',
                                 Color::new(222, 184, 135),
                                 Color::new(50, 200, 50));
            }
            Bermudagrass => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 ';',
                                 Color::new(124, 252, 0),
                                 Color::new(50, 200, 50));
            }
            Bindweed => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 ',',
                                 Color::new(75, 0, 130),
                                 Color::new(50, 200, 50));
            }

            Crabapple => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::CLUB,
                                 Color::new(186, 85, 211),
                                 Color::new(50, 200, 50));
            }
            Redbud => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 chars::CLUB,
                                 Color::new(216, 191, 216),
                                 Color::new(50, 200, 50));
            }
            CrapeMyrtle => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }

            AcerRubrum => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }
            RedBuckeye => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }
            MinnesotaStrainRedbud => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }

            BaldCypress => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }
            BoxElder => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }
            ChineseTallowTree => {
                root.put_char_ex(pos.0,
                                 pos.1,
                                 '!',
                                 Color::new(255, 255, 254),
                                 Color::new(50, 200, 50));
            }
        }
    }
}

/////// BIOME

pub struct Biome {
    temperature_high_f: i32,
    temperature_low_f: i32,
    name: &'static str,
    humidity_pcnt: i32,
    percipitation_chance: i32,
}

/////// GENERAL
// State: the 3 physical forms + fire because it's convenient.
pub enum State {
    Liquid,
    Solid,
    Gas,
    Flaming,
}

// Descriptive alias (hey, I'm a haskell programmer).
pub type Height = i32;

// General types of tiles (very broad) and their current state.
pub enum Tile {
    Empty,
    Water(LiquidPurity, State),
    Stone(StoneTypes, State),
    Vegitation(VegTypes, Height, State),
}

// A unit is a 1x1 cross section of the layered world, including a ref
// to the biome its part of.
// Each tile is 1 foot tall.
pub struct Unit {
    tiles: Vec<Tile>,
    biomes: Vec<Biome>,
}

pub type World = Vec<Vec<Unit>>;
impl World {
    fn biome_from_height(height: isize) -> Biome {
        Biome {
            temperature_high_f: 75,
            temperature_low_f: 52,
            name: "LowMed",
            humidity_pcnt: 60,
            percipitation_chance: 70,
        }
    }

    fn rock_type(rock_num: isize, height: isize) -> Tile {
        Tile::Stone(StoneTypes::Sedimentary(SedimentaryRocks::Limestone),
                    State::Solid)
    }

    fn vegetation_from_noise(veg_num: isize) -> Tile {
        Tile::Vegitation(VegTypes::Bluegrass, 1, State::Solid)
    }

    pub fn new(size: (usize, usize), seed: u32) -> World {
        println!("Generating seed from {}", seed);

        // Geology
        let wnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.2)
            .hurst(0.6)
            .noise_type(NoiseType::Perlin)
            .random(Rng::new_with_seed(Algo::MT, seed))
            .init();

        // Vegitation
        let vnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(0.8)
            .noise_type(NoiseType::Perlin)
            .random(Rng::new_with_seed(Algo::MT, seed))
            .init();

        // Rock type
        let rnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(0.8)
            .noise_type(NoiseType::Perlin)
            .random(Rng::new_with_seed(Algo::MT, seed))
            .init();

        let (sx, sy) = size;
        let mut heightmap = vec![vec![0f32; sx]; sy];
        for y in 0..sy {
            for x in 0..sx {
                heightmap[y][x] = wnoise.get_fbm(&mut [x as f32, y as f32],
                                                 3);
            }
        }

        let mut world: World = vec![];
        for y in (0..sy).rev() {
            let mut line = vec![];
            for x in (0..sx).rev() {
                let mut tiles: Vec<Tile> = vec![];
                let mut biomes: Vec<Biome> = vec![];

                for z in 0..(heightmap[y][x] as isize - 1) {
                    biomes.push(World::biome_from_height(z));

                    let rn = rnoise.get_fbm(&mut [x as f32, y as f32], 2) as
                        isize;
                    tiles.push(World::rock_type(rn, z));
                }

                let vn = vnoise.get_fbm(&mut [x as f32, y as f32], 2) as
                    isize;
                tiles.push(World::vegetation_from_noise(vn));

                line.push(Unit {
                    tiles: tiles,
                    biomes: biomes,
                });
            }
            world.push(line);
        }

        world
    }
}

pub struct WorldState {
    screen: (i32, i32),
    cursor: (i32, i32),
    level: i32,
    animals: Vec<Animal>,
    map: World,
}
impl WorldState {
    pub fn new(world: World) -> WorldState {
        WorldState {
            screen: (0, 0),
            level: 0,
            cursor: (0, 0),
            animals: vec![],
            map: world,
        }
    }
}
