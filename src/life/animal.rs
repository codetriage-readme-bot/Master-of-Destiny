extern crate rand;
use self::rand::Rng;

use std;

use life::{Living, Mission, MissionResult};
use utils::{Point3D, distance, find_path, nearest_perimeter_point,
            random_point, strict_adjacent};
use worldgen::World;
use physics::PhysicsActor;
use worldgen::terrain::{Biome, BiomeType, Food, Item, Tile, VegType};

const THIRST_THRESHOLD: i32 = 3000;
const HUNGER_THRESHOLD: i32 = 6800;

macro_rules! matches {
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Carnivore {
    Dog,
    Cat,
    Wolf,
    Shark,
    Alligator,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Herbivore {
    Cow,
    Sheep,
    Hippo,
    Rabbit,
    Armadillo,
    Fish,
    Whale,
}

/// Possible animal species.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Species {
    Carnivore(Carnivore),
    Herbivore(Herbivore),
}

pub struct SpeciesProperties {
    pub health: i32,
    pub chr: char,
    pub sight: u8,
    pub mood: super::Mood,
    pub species: Species,
}

impl SpeciesProperties {
    pub fn can_go(&self, biome: Biome) -> bool {
        use self::Species::*;
        use self::Herbivore::*;
        use self::Carnivore::*;

        let b = biome.biome_type;
        match self.species {
            Herbivore(Whale) => b == BiomeType::Water,
            Herbivore(Fish) => b == BiomeType::Water,
            Carnivore(Shark) => b == BiomeType::Water,
            Carnivore(Dog) => b != BiomeType::Water,
            Carnivore(Cat) => b != BiomeType::Water,
            Carnivore(Wolf) => b != BiomeType::Water,
            Herbivore(_) => b != BiomeType::Water,
            _ => true,
        }
    }
}

impl Species {
    pub fn properties(&self) -> SpeciesProperties {
        use self::Species::*;
        use self::Carnivore::*;
        use self::Herbivore::*;

        match self {
            &Carnivore(Dog) => {
                SpeciesProperties {
                    health: 200,
                    chr: 'd',
                    sight: 24,
                    species: self.clone(),
                    mood: super::Mood::Contented,
                }
            }
            &Carnivore(Cat) => {
                SpeciesProperties {
                    health: 100,
                    chr: 'c',
                    species: self.clone(),
                    sight: 27,
                    mood: super::Mood::Discontented,
                }
            }
            &Carnivore(Wolf) => {
                SpeciesProperties {
                    health: 400,
                    chr: 'w',
                    species: self.clone(),
                    sight: 33,
                    mood: super::Mood::Wary,
                }
            }
            &Carnivore(Shark) => {
                SpeciesProperties {
                    health: 800,
                    chr: 'S',
                    species: self.clone(),
                    sight: 60,
                    mood: super::Mood::Agressive,
                }
            }
            &Carnivore(Alligator) => {
                SpeciesProperties {
                    health: 800,
                    chr: 'A',
                    species: self.clone(),
                    sight: 60,
                    mood: super::Mood::Agressive,
                }
            }
            &Herbivore(Fish) => {
                SpeciesProperties {
                    health: 10,
                    species: self.clone(),
                    chr: 'f',
                    sight: 9,
                    mood: super::Mood::Fearful,
                }
            }
            &Herbivore(Whale) => {
                SpeciesProperties {
                    health: 10000,
                    chr: 'W',
                    species: self.clone(),
                    sight: 45,
                    mood: super::Mood::Contented,
                }
            }

            &Herbivore(Cow) => {
                SpeciesProperties {
                    health: 300,
                    chr: 'c',
                    species: self.clone(),
                    sight: 15,
                    mood: super::Mood::Contented,
                }
            }
            &Herbivore(Hippo) => {
                SpeciesProperties {
                    health: 300,
                    chr: 'H',
                    species: self.clone(),
                    sight: 15,
                    mood: super::Mood::Agressive,
                }
            }
            &Herbivore(Sheep) => {
                SpeciesProperties {
                    health: 200,
                    chr: 's',
                    species: self.clone(),
                    sight: 6,
                    mood: super::Mood::Wary,
                }
            }
            &Herbivore(Rabbit) => {
                SpeciesProperties {
                    health: 10,
                    chr: 'r',
                    species: self.clone(),
                    sight: 9,
                    mood: super::Mood::Fearful,
                }
            }
            &Herbivore(Armadillo) => {
                SpeciesProperties {
                    health: 100,
                    chr: 'a',
                    species: self.clone(),
                    sight: 15,
                    mood: super::Mood::Wary,
                }
            }
        }
    }
}

/// The animal itself. It keeps track of all its mental and physical states, as well as its goals.
pub struct Animal {
    thirst: i32,
    hunger: i32,
    goals: Vec<super::Mission>,
    path: Option<Vec<Point3D>>,
    arrived: bool,
    failed_goal: Option<super::Mission>,
    pub species: SpeciesProperties,
    pub pos: (usize, usize, usize),
    pub current_goal: Option<super::Mission>,
}

impl Animal {
    pub fn new(pnt: Point3D, species: Species) -> Box<super::Living> {
        Box::new(Animal {
                     thirst: 200,
                     hunger: 600,
                     goals: vec![],
                     path: None,
                     arrived: false,
                     failed_goal: None,
                     pos: pnt,
                     current_goal: None,
                     species: species.properties(),
                 })
    }

    fn in_sight(&self, map: &World) -> Vec<(Tile, Point3D)> {
        let r = self.species.sight as i32;
        let mut adj = vec![];
        for x in (-r)..r {
            for y in (-r)..r {
                if x * x + y * y <= r * r {
                    let maybe_unit =
                        map.get((x as usize, y as usize));
                    adj.push(if let Some(unit) = maybe_unit {
                        let tiles = unit.tiles.borrow();
                        (
                            tiles
                                .get(self.pos.2)
                                .unwrap_or(
                                    tiles
                                        .get(std::cmp::max(0, tiles.len() as i32 - 1) as usize)
                                        .unwrap_or(&Tile::Empty),
                                )
                                .clone(),
                            (
                                x as usize,
                                y as usize,
                                std::cmp::min(self.pos.2, tiles.len()),
                            ),
                        )
                    } else {
                        (Tile::Empty, (x as usize, y as usize, 0))
                    })
                }
            }
        }
        adj
    }

    fn add_path_to_point(&mut self, map: &World, pnt: Point3D, mission: Mission) {
        if let Some(path) = self.create_path_to(map, pnt) {
            self.path = Some(path);
        } else {
            self.current_goal = None;
            self.failed_goal = Some(mission);
        }
    }

    fn set_up_new_goal(&mut self, map: &World,
                       in_sight: Vec<(Tile, Point3D)>) -> MissionResult {
        use self::Mission::*;
        if let Some(m) = self.current_goal {
        for (tile, pnt) in in_sight {
            // Set up to complete the current goal.
                let s = self.species.species;
                return match m {
                    // If we need to eat, find a food item nearby to eat.
                    Eat(_) => {
                        let carnivore_food = matches!(s, Species::Carnivore(..)) &&
                            matches!(tile, Tile::Item(Item::Food(Food::Meat(..))));
                        let herbivore_food = matches!(s, Species::Herbivore(..)) &&
                            matches!(tile, Tile::Item(Item::Food(Food::Herb(..))));
                        if carnivore_food || herbivore_food {
                            self.add_path_to_point(map, pnt, m);
                        }
                        MissionResult::NoResult
                    }
                    // If we need to drink, find the shoreline.
                    Drink(_) => {
                        let mut trng = self::rand::thread_rng();
                        if let Some(pnt) = trng.choose(&map.biome_map["w"].borrow()) {
                            let adj = strict_adjacent(*pnt);
                            let shore = adj.iter().find(|pnt| {
                                let map = map.get(**pnt).unwrap();
                                let ut = map.tiles.borrow();
                                ut.iter().enumerate().find(
                                    |&(i, tile)| !tile.solid() && ut[i - 1].solid()
                                ).is_some()
                            });
                            if let Some(&(x, y)) = shore {
                                self.add_path_to_point(
                                    map,
                                    (x, y, map.location_z((x, y))),
                                    m
                                );
                            }
                        }
                        MissionResult::NoResult
                    }
                    // Find an enemy to attack, not of the same species
                    AttackEnemy(_) => {
                        for l in map.life.iter() {
                            let pos = l.borrow().current_pos();
                            let dist = distance((pos.0, pos.1),
                                                (self.pos.0,
                                                 self.pos.1));
                            if pos.2 == self.pos.2 &&
                                dist <=
                                self.species.sight as f32 &&
                                l.borrow().species().species !=
                                self.species.species
                            {
                                self.add_path_to_point(map, pnt, m);
                            }
                        }
                        MissionResult::NoResult
                    }
                    // Go to a gathering area
                    GoToArea(rect, _) => {
                        let p = self.pos.clone();
                        self.add_path_to_point(
                            map,
                            nearest_perimeter_point(rect, p),
                            m
                        );
                        MissionResult::NoResult
                    }
                    // go to a point
                    Go(point, _) => {
                        self.add_path_to_point(
                            map,
                            (point.0, point.1, map.location_z(point)),
                            m
                        );
                        MissionResult::NoResult
                    }
                    // Die
                    Mission::Die => {
                        println!("Executing death as mission");
                        MissionResult::Die
                    }
                    _ => MissionResult::NoResult
                };
            }
        } else {
            self.auto_add_mission(map, in_sight);
            return MissionResult::NoResult;
        }
        self.failed_goal = self.current_goal;
        self.current_goal = None;
        MissionResult::NoResult
    }

    fn satisfy_current_goal(&mut self, map: &World) -> MissionResult {
        let in_sight = self.in_sight(map);

        if self.path.is_none() && !self.arrived {
            // If we're not in the process of doing anything right now...
            self.set_up_new_goal(map, in_sight)
        } else if self.path.is_some() {
            self.hunger += 10;
            self.thirst += 10;
            self.continue_movement(map);
            MissionResult::NoResult
        } else if self.arrived {
            self.thirst += 10;
            self.stationary_action(map, in_sight)
        } else {
            MissionResult::NoResult
        }
    }

    fn create_path_to(&self,
                      map: &World,
                      goal: Point3D)
        -> Option<Vec<Point3D>> {
        let unit = map.get((goal.0, goal.1)).unwrap();
        if unit.biome.is_some() &&
            self.species.can_go(unit.biome.unwrap())
        {
            find_path(map, self.pos, goal, box |point| {
                let b = map.get((point.0, point.1))
                           .unwrap()
                           .biome;
                return b.is_some() && self.species.can_go(b.unwrap());
            })
        } else {
            None
        }
    }

    fn continue_movement(&mut self, _map: &World) {
        if let Some(ref mut path) = self.path {
            if let Some(npos) = path.pop() {
                self.pos = npos;
            } else {
                self.arrived = true;
            }
        }
    }

    fn stationary_action(&mut self,
                         map: &World,
                         adj: Vec<(Tile, Point3D)>)
        -> MissionResult {
        let result = match self.current_goal {
            Some(Mission::AttackEnemy(_)) => {
                let enemy = map.life
                               .iter()
                               .enumerate()
                               .find(|&(_i, e)| {
                    adj.iter()
                       .find(|&&(_, p)| e.borrow().current_pos() == p)
                       .is_some()
                });
                if let Some((i, _)) = enemy {
                    MissionResult::Kill(i)
                } else {
                    MissionResult::NoResult
                }
            }
            Some(Mission::Drink(_)) => {
                if let Some(&(Tile::Item(Item::Food(Food::Water(q))), pnt)) =
                    adj.iter().find(
                        |&&(t, _)| matches!(t, Tile::Item(Item::Food(..))),
                    )
                {
                    self.thirst /= 4;
                    MissionResult::ReplaceItem(pnt, Item::Food(Food::Water(q / 4)))
                } else {
                    MissionResult::NoResult
                }
            }
            Some(Mission::Eat(p)) => {
                if let Some(&(Tile::Item(Item::Food(food)), pnt)) =
                    adj.iter().find(|&&(t, _)| {
                        matches!(t, Tile::Item(Item::Food(..)))
                    })
                {
                    match self.species.species {
                        Species::Herbivore(_) => {
                            if let Food::Herb(_) = food {
                                self.hunger /= 2;
                                MissionResult::RemoveItem(pnt)
                            } else {
                                self.failed_goal =
                                    Some(Mission::Eat(p));
                                MissionResult::NoResult
                            }
                        }
                        Species::Carnivore(_) => {
                            if let Food::Meat(species) = food {
                                if species != self.species.species {
                                    self.hunger = 0;
                                    MissionResult::RemoveItem(pnt)
                                } else {
                                    self.failed_goal =
                                        Some(Mission::Eat(p));
                                    MissionResult::NoResult
                                }
                            } else {
                                self.failed_goal =
                                    Some(Mission::Eat(p));
                                MissionResult::NoResult
                            }
                        }
                    }
                } else {
                    self.failed_goal = Some(Mission::Eat(p));
                    MissionResult::NoResult
                }
            }
            Some(Mission::Die) => {
                println!("Executing death at stationary.");
                return MissionResult::Die;
            }
            _ => MissionResult::NoResult,
        };

        self.current_goal = self.goals.pop();
        result
    }

    fn tolerance(&self) -> i32 { 800 }
}

impl Living for Animal {
    fn add_goal(&mut self, mission: Mission) {
        if matches!(mission, Mission::Die) {
            println!("Adding death mission.");
            self.failed_goal = None;
            self.current_goal = Some(Mission::Die);
            self.species.health = 0;
            self.goals = vec![];
        } else {
            self.failed_goal = None;
            match self.goals.binary_search(&mission) {
                Ok(_) => {}
                Err(i) => {
                    self.goals.insert(i, mission);
                    if self.current_goal.is_none() {
                        self.current_goal = Some(self.goals[i]
                                                     .clone());
                    }
                }
            };
        }
    }

    fn remove_goal(&mut self, tag: &Mission) -> Option<Mission> {
        self.goals.remove_item(tag)
    }

    fn prioritize(&mut self, n: usize) -> Vec<Mission> {
        if n <= self.goals.len() {
            self.goals.drain(0..n).collect()
        } else {
            self.goals.clone()
        }
    }

    fn execute_mission(&mut self, map: &World) -> MissionResult {
        if self.current_goal.is_some() {
            self.satisfy_current_goal(map)
        } else {
            let m = self.goals.pop();
            if m.is_some() {
                self.current_goal = m;
            } else {
                let is = self.in_sight(map);
                self.auto_add_mission(map, is);
            }
            self.satisfy_current_goal(map)
        }
    }

    fn auto_add_mission(&mut self, map: &World,
                        adj: Vec<(Tile, Point3D)>) -> Option<Mission> {
        if self.thirst >= THIRST_THRESHOLD ||
            self.hunger >= HUNGER_THRESHOLD
        {
            println!("Died of hunger or thirst");
            self.add_goal(Mission::Die);
            return Some(Mission::Die);
        }
        let thirsty = self.thirst >=
            THIRST_THRESHOLD - self.tolerance();
        if thirsty {
            let magnitude = (THIRST_THRESHOLD - self.thirst)
                .abs() as usize;
            self.add_goal(Mission::Drink(magnitude));
        }
        let hungry = self.hunger >=
            HUNGER_THRESHOLD - self.tolerance();
        if hungry {
            let magnitude = (HUNGER_THRESHOLD - self.hunger)
                .abs() as usize;
            self.add_goal(Mission::Eat(magnitude));
        }
        match &self.failed_goal {
                &Some(Mission::Eat(p)) => {
                    if matches!(self.species.species,
                                Species::Carnivore(..)) {
                        self.add_goal(Mission::AttackEnemy(p));
                    } else {
                        let (x, y, z) = self.current_pos();
                        let plant = adj.iter().find(|&&(tile, _)|
                                                           matches!(tile,
                                                                    Tile::Vegetation(..)));
                        if let Some(&(Tile::Vegetation(vt, ..), (x, y, z))) = plant {
                            let unit = map.get((x, y)).unwrap();
                            let mut ut = unit.tiles.borrow_mut();
                            ut[z] = Tile::Item(Item::Food(Food::Herb(vt)));
                        }
                    }
                }
                _ => {
                    self.current_goal = self.goals.pop();
                    self.failed_goal = None;
                }
        }
        if self.goals.len() == 0 || (hungry || thirsty) {
            match self.species.species {
                // Fish group to confuse enemies
                Species::Herbivore(Herbivore::Fish) => {
                    let mut trng = self::rand::thread_rng();
                    let fish =
                        &map.life
                            .iter()
                            .filter_map(
                            |a| if let Ok(a) = a.try_borrow() {
                                if a.species().species ==
                            Species::Herbivore(Herbivore::Fish)
                        {
                            Some(a.current_pos())
                        } else {
                            None
                        }
                            } else {
                                None
                            },
                        )
                            .collect::<Vec<_>>();
                    for l in map.life.iter() {
                        let other = l.borrow();
                        let pos = other.current_pos();
                        let dist = distance((pos.0, pos.1),
                                            (self.pos.0,
                                             self.pos.1));
                        if pos.2 <= self.pos.2 &&
                            dist <=
                            self.species.sight as f32 &&
                            other.species().species !=
                            self.species.species &&
                            matches!(other.species().species, Species::Carnivore(..))
                        {
                            if let Some(goal) = trng.choose(&fish) {
                                self.add_goal(Mission::Go((goal.0, goal.1), 0));
                            }
                        }
                    }
                }
                // Whales like to form schools. That should become an
                // emergant property of this.
                Species::Herbivore(Herbivore::Whale) => {
                    let mut trng = self::rand::thread_rng();
                    let whales =
                        &map.life
                            .iter()
                            .filter_map(
                            |a| if let Ok(a) = a.try_borrow() {
                                if a.species().species ==
                            Species::Herbivore(Herbivore::Whale)
                        {
                            Some(a.current_pos())
                        } else {
                            None
                        }
                            } else {
                                None
                            },
                        )
                            .collect::<Vec<_>>();
                    if let Some(goal) = trng.choose(whales) {
                        self.add_goal(Mission::Go((goal.0, goal.1), 10));
                    } else {
                        self.add_goal(
                            Mission::Go(random_point(0,
                                                     map.map_size.0,
                                                     0,
                                                     map.map_size.1),
                                        12));
                    }
                }
                _ => {
                    self.add_goal(
                        Mission::Go(random_point(0,
                                                 map.map_size.0,
                                                 0,
                                                 map.map_size.1),
                                    12));
                }
            }
        }
        self.current_goal
    }

    fn current_pos(&self) -> Point3D { self.pos }
    fn species(&self) -> &SpeciesProperties { &self.species }
    fn goals(&self) -> (Option<&Mission>, Option<&Mission>) {
        (self.current_goal.as_ref(), self.failed_goal.as_ref())
    }
}
