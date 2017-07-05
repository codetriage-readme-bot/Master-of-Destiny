extern crate rand;

use draw::DrawChar;
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

fn adjacent((ux, uy): (usize, usize),
            size: (usize, usize))
    -> Vec<(usize, usize)> {
    let (x, y) = (ux as i32, uy as i32);
    let (width, height) = (size.0 as i32, size.1 as i32);
    let fx = |x: i32| clamp(x, width - 1, 0) as usize;
    let fy = |y: i32| clamp(y, height - 1, 0) as usize;
    let fxy = |x, y| (fx(x), fy(y));

    vec![fxy(x + 1, y),
         fxy(x - 1, y),
         fxy(x, y + 1),
         fxy(x, y - 1),
         fxy(x + 1, y + 1),
         fxy(x - 1, y - 1),
         fxy(x + 1, y - 1),
         fxy(x - 1, y + 1)]
}

/////// ROCK
// Possible igneous rock kinds
#[derive(Debug)]
pub enum IgneousRocks {
    Obsidian,
    Basalt,
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
#[derive(Debug)]
pub enum MetamorphicRocks {
    Gneiss,
    Marble,
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
#[derive(Debug)]
pub enum SedimentaryRocks {
    Limestone,
    Conglomerate,
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

// Stone types (SCIENCE!)
#[derive(Debug)]
pub enum StoneTypes {
    Sedimentary(SedimentaryRocks),
    Igneous(IgneousRocks),
    Metamorphic(MetamorphicRocks),
}

impl DrawChar for StoneTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &StoneTypes::Sedimentary(ref s) => s.draw_char(root, pos),
            &StoneTypes::Metamorphic(ref s) => s.draw_char(root, pos),
            &StoneTypes::Igneous(ref s) => s.draw_char(root, pos),
        }
    }
}

/////// WATER
// This is a DF-type game, so... extra fidelty!
#[derive(Debug)]
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
    // Tall trees (height 10-20)
    Pine,
    Redwood,
    Banyon,
}

impl DrawChar for VegTypes {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        match self {
            &VegTypes::Bluegrass => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '"',
                                 Color::new(0, 200, 150),
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
                                 chars::SPADE,
                                 Color::new(255, 255, 250),
                                 Color::new(50, 200, 50));
            }
            &VegTypes::Redwood => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 chars::ARROW_N,
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

pub struct Biome {
    temperature_high_f: i32,
    temperature_low_f: i32,
    name: &'static str,
    humidity_pcnt: i32,
    percipitation_chance: i32,
}

/////// GENERAL
// State: the 3 physical forms + fire because it's convenient.
#[derive(Debug)]
pub enum State {
    Liquid,
    Solid,
    Gas,
}

// Descriptive alias (hey, I'm a haskell programmer).
pub type Height = i32;

// North is up, South is down, East is left, West is right.
#[derive(Debug)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum Slope {
    Up,
    Down,
}

// General types of tiles (very broad) and their current state.
#[derive(Debug)]
pub enum Tile {
    Empty,
    Ramp(Box<Tile>, Slope),
    Moveable(Box<Tile>),
    Water(LiquidPurity, State),
    Stone(StoneTypes, State),
    Vegitation(VegTypes, Height, State),
    Fire,
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
                    &box Tile::Stone(ref s, State::Solid) => {
                        s.draw_char(root, pos);
                        root.put_char(pos.0 as i32,
                                      pos.1 as i32,
                                      chars::BULLET,
                                      BackgroundFlag::Set);
                    }
                    &box Tile::Vegitation(ref v, ..) => {
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
                    &box Tile::Moveable(..) => panic!("Nested moveables!"),
                    _ => panic!("Shouldn't be moveable!"),
                }
            }
            &Tile::Stone(ref s, State::Solid) => s.draw_char(root, pos),
            &Tile::Stone(_, State::Liquid) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '~',
                                 Color::new(255, 0, 0),
                                 Color::new(0, 0, 0));
            }
            &Tile::Stone(_, State::Gas) => panic!("Stones can't be a gas!"),
            &Tile::Water(_, State::Solid) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '#',
                                 Color::new(255, 255, 255),
                                 Color::new(100, 255, 100));
            }
            &Tile::Water(_, State::Liquid) => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 '\u{f7}',
                                 Color::new(0, 105, 148),
                                 Color::new(0, 159, 225));
            }
            &Tile::Water(_, State::Gas) => {
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
            &Tile::Empty => {
                root.put_char_ex(pos.0 as i32,
                                 pos.1 as i32,
                                 ' ',
                                 Color::new(135, 206, 250),
                                 Color::new(135, 206, 250))
            }
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
    pub world_noise: Noise,
    pub vegetation_noise: Noise,
    pub stone_vein_noise: Noise,
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

const THRESHOLD: f32 = 80.0;
impl World {
    pub fn new(size: (usize, usize), seed: u32) -> World {
        println!("Generating seed from {}", seed);
        let wnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.1)
            .hurst(0.7)
            .noise_type(NoiseType::Perlin)
            .random(random::Rng::new_with_seed(random::Algo::MT, seed))
            .init();

        // Vegitation
        let vnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.3)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT, seed))
            .init();

        // Rock type
        let rnoise = Noise::init_with_dimensions(2)
            .lacunarity(0.43)
            .hurst(-0.9)
            .noise_type(NoiseType::Simplex)
            .random(random::Rng::new_with_seed(random::Algo::MT, seed))
            .init();

        let (sx, sy) = size;
        let mut heightmap = vec![vec![0f32; sx]; sy];
        for y in 0..sy {
            for x in 0..sx {
                heightmap[y][x] =
                    wnoise.get_fbm(&mut [x as f32, y as f32], 2) *
                        THRESHOLD;
            }
        }

        let mut world: World = World {
            map: vec![],
            world_noise: wnoise,
            vegetation_noise: vnoise,
            stone_vein_noise: rnoise,
        };
        for y in 0..sy {
            let mut line = vec![];
            for x in 0..sx {
                let mut tiles: Vec<Tile> = vec![];
                let mut biomes: Vec<Biome> = vec![];
                for n in (0..30).rev() {
                    tiles.push(world.rock_type((x, y), -n));
                }

                let height = heightmap[y][x];
                if height <= 30.0 {
                    if heightmap[y][x] > 2.0 {
                        tiles.push(Tile::Ramp(box world.get_vegetation((x, y)), Slope::Up));
                    } else {
                        tiles.push(world.get_vegetation((x, y)));
                    }
                    for _ in 0..((heightmap[y][x] - 1.0) as i32) {
                        tiles.push(world.get_vegetation((x, y)));
                    }
                    if adjacent((x, y), (sx, sy))
                        .iter()
                        .find(|z| heightmap[z.1][z.0] > height)
                        .is_some()
                    {
                        tiles.push(Tile::Ramp(box world.get_vegetation((x, y)),
                                              Slope::Up));
                    }
                    if adjacent((x, y), (sx, sy))
                        .iter()
                        .find(|z| heightmap[z.1][z.0] < height)
                        .is_some()
                    {
                        tiles.push(Tile::Ramp(box world.get_vegetation((x, y)),
                                              Slope::Down));
                    }
                } else {
                    for z in 0..(height as isize - 1) {
                        biomes.push(world.biome_from_height(z));
                        tiles.push(world.rock_type((x, y), z));
                    }
                    tiles.push(world.get_vegetation((x, y)));
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

    pub fn purity(vein: isize) -> LiquidPurity { LiquidPurity::Clean }

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
            name: "LowMed",
            humidity_pcnt: 60,
            percipitation_chance: 70,
        }
    }

    pub fn rock_type(&self, (x, y): (usize, usize), height: isize) -> Tile {
        let rn =
            self.stone_vein_noise
                .get_turbulence(&mut [x as f32, y as f32], 3) as
                isize;
        Tile::Stone(StoneTypes::Sedimentary(SedimentaryRocks::Limestone),
                    State::Solid)
    }

    pub fn get_vegetation(&self, (x, y): (usize, usize)) -> Tile {
        let vn = self.vegetation_noise
                     .get_fbm(&mut [x as f32, y as f32], 1);
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
