extern crate rand;
use self::rand::Rng;

use std;

use life::{Living, Mission, MissionResult};
use utils::{Point3D, distance, find_path, nearest_perimeter_point,
            random_point};
use worldgen::World;
use worldgen::terrain::{Biome, BiomeType, Food, IgneousRocks, Item,
                        State, StoneTypes, Tile};

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
    Fish,
    Whale,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Herbivore {
    Cow,
    Sheep,
    Hippo,
    Rabbit,
    Armadillo,
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
        use self::Carnivore::*;

        let b = biome.biome_type;
        match self.species {
            Carnivore(Whale) => b == BiomeType::Water,
            Carnivore(Fish) => b == BiomeType::Water,
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
                    sight: 8,
                    species: self.clone(),
                    mood: super::Mood::Contented,
                }
            }
            &Carnivore(Cat) => {
                SpeciesProperties {
                    health: 100,
                    chr: 'c',
                    species: self.clone(),
                    sight: 9,
                    mood: super::Mood::Discontented,
                }
            }
            &Carnivore(Wolf) => {
                SpeciesProperties {
                    health: 400,
                    chr: 'w',
                    species: self.clone(),
                    sight: 11,
                    mood: super::Mood::Wary,
                }
            }
            &Carnivore(Shark) => {
                SpeciesProperties {
                    health: 800,
                    chr: 'S',
                    species: self.clone(),
                    sight: 20,
                    mood: super::Mood::Agressive,
                }
            }
            &Carnivore(Alligator) => {
                SpeciesProperties {
                    health: 800,
                    chr: 'A',
                    species: self.clone(),
                    sight: 20,
                    mood: super::Mood::Agressive,
                }
            }
            &Carnivore(Fish) => {
                SpeciesProperties {
                    health: 10,
                    species: self.clone(),
                    chr: 'f',
                    sight: 3,
                    mood: super::Mood::Fearful,
                }
            }
            &Carnivore(Whale) => {
                SpeciesProperties {
                    health: 10000,
                    chr: 'W',
                    species: self.clone(),
                    sight: 15,
                    mood: super::Mood::Contented,
                }
            }

            &Herbivore(Cow) => {
                SpeciesProperties {
                    health: 300,
                    chr: 'c',
                    species: self.clone(),
                    sight: 5,
                    mood: super::Mood::Contented,
                }
            }
            &Herbivore(Hippo) => {
                SpeciesProperties {
                    health: 300,
                    chr: 'H',
                    species: self.clone(),
                    sight: 5,
                    mood: super::Mood::Agressive,
                }
            }
            &Herbivore(Sheep) => {
                SpeciesProperties {
                    health: 200,
                    chr: 's',
                    species: self.clone(),
                    sight: 2,
                    mood: super::Mood::Wary,
                }
            }
            &Herbivore(Rabbit) => {
                SpeciesProperties {
                    health: 10,
                    chr: 'r',
                    species: self.clone(),
                    sight: 3,
                    mood: super::Mood::Fearful,
                }
            }
            &Herbivore(Armadillo) => {
                SpeciesProperties {
                    health: 100,
                    chr: 'a',
                    species: self.clone(),
                    sight: 5,
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
                     thirst: 30,
                     hunger: 0,
                     goals: vec![],
                     path: None,
                     arrived: false,
                     failed_goal: None,
                     pos: pnt,
                     current_goal: None,
                     species: species.properties(),
                 })
    }

    fn satisfy_current_goal(&mut self, map: &World) -> MissionResult {
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
        self.step_to_goal(map, adj)
    }

    fn step_to_goal(&mut self,
                    map: &World,
                    adj: Vec<(Tile, Point3D)>)
        -> MissionResult {
        use self::Mission::*;
        if self.path.is_none() && !self.arrived {
            for (tile, pnt) in adj {
                if let Some(m) = self.current_goal.clone() {
                    let s = self.species.species;
                    match m {
                        Eat(_) => {
                            let carnivore_food = matches!(s, Species::Carnivore(..)) &&
                                matches!(tile, Tile::Item(Item::Food(Food::Meat(..))));
                            let herbivore_food = matches!(s, Species::Herbivore(..)) &&
                                matches!(tile, Tile::Item(Item::Food(Food::Herb(..))));
                            if carnivore_food || herbivore_food {
                                let path =
                                    self.create_path_to(map, pnt);
                                if path.is_none() {
                                    self.current_goal = None;
                                    self.failed_goal = Some(m);
                                } else {
                                    self.path = path;
                                }
                                return MissionResult::NoResult;
                            }
                        }
                        Drink(_) => {
                            if matches!(tile, Tile::Item(Item::Food(Food::Water(..)))) {
                                let path = self.create_path_to(map, pnt);
                                if path.is_none() {
                                    self.current_goal = None;
                                    self.failed_goal = Some(m);
                                } else {
                                    self.path = path;
                                }
                                return MissionResult::NoResult;
                            }
                        }
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
                                    let path =
                                        self.create_path_to(map, pnt);
                                    if path.is_none() {
                                        self.current_goal = None;
                                        self.failed_goal = Some(m);
                                    } else {
                                        self.path = path;
                                    }
                                    return MissionResult::NoResult;
                                }
                            }
                        }
                        GoToArea(rect, _) => {
                            let p = self.pos.clone();
                            let path = self.create_path_to(map, nearest_perimeter_point(rect, p));
                            if path.is_none() {
                                self.current_goal = None;
                                self.failed_goal = Some(m);
                            } else {
                                self.path = path;
                            }
                            return MissionResult::NoResult;
                        }
                        Go(point, _) => {
                            let path =
                                self.create_path_to(map, (point.0, point.1, map.location_z(point)));
                            if path.is_none() {
                                self.current_goal = None;
                                self.failed_goal = Some(m);
                            } else {
                                self.path = path;
                            }
                            return MissionResult::NoResult;
                        }
                        _ => {}
                    }
                } else {
                    self.auto_add_mission(map);
                }
            }
            self.failed_goal = self.current_goal.clone();
            self.current_goal = None;
        } else if self.path.is_some() {
            self.hunger += 10;
            self.thirst += 10;
            self.continue_movement(map);
        } else if self.arrived {
            self.thirst += 10;
            return self.stationary_action(map, adj);
        }
        MissionResult::NoResult
    }

    fn create_path_to(&self,
                      map: &World,
                      goal: Point3D)
        -> Option<Vec<Point3D>> {
        let unit = map.get((goal.0, goal.1)).unwrap();
        if self.species.can_go(unit.biome.unwrap()) {
            find_path(map, self.pos, goal)
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
        println!("{:?}", self.goals);
        if self.current_goal.is_some() {
            self.satisfy_current_goal(map)
        } else {
            let m = self.goals.pop();
            if m.is_some() {
                self.current_goal = m;
            } else {
                self.auto_add_mission(map);
            }
            self.satisfy_current_goal(map)
        }
    }

    fn auto_add_mission(&mut self, map: &World) -> Option<Mission> {
        if self.thirst >= THIRST_THRESHOLD ||
            self.hunger >= HUNGER_THRESHOLD
        {
            return Some(Mission::Die);
        }
        if self.thirst >= THIRST_THRESHOLD - self.tolerance() {
            let magnitude = (THIRST_THRESHOLD - self.thirst)
                .abs() as usize;
            self.add_goal(Mission::Drink(magnitude));
        }
        if self.hunger >= HUNGER_THRESHOLD - self.tolerance() {
            let magnitude = (HUNGER_THRESHOLD - self.hunger)
                .abs() as usize;
            self.add_goal(Mission::Eat(magnitude));
        }
        if let Some(g) = self.failed_goal.clone() {
            match g {
                Mission::Eat(p) => {
                    if matches!(self.species.species,
                                Species::Carnivore(..))
                    {
                        self.add_goal(Mission::AttackEnemy(p));
                    }
                }
                _ => {
                    self.current_goal = self.goals.pop();
                    self.failed_goal = None;
                }
            }
        }
        println!("{:?}", self.goals);
        if self.goals.len() == 0 {
            match self.species.species {
                // Whales like to form schools. That should become an
                // emergant property of this.
                Species::Carnivore(Carnivore::Whale) => {
                    let mut trng = self::rand::thread_rng();
                    let whales =
                        &map.life
                            .iter()
                            .filter_map(
                            |a| if let Ok(a) = a.try_borrow() {
                                if a.species().species ==
                            Species::Carnivore(Carnivore::Whale)
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
                    let goal = trng.choose(whales).unwrap();
                    self.add_goal(Mission::Go((goal.0, goal.1), 10));
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
        self.current_goal.clone()
    }

    fn current_pos(&self) -> Point3D { self.pos }
    fn species(&self) -> &SpeciesProperties { &self.species }
    fn goals(&self) -> (Option<&Mission>, Option<&Mission>) {
        (self.current_goal.as_ref(), self.failed_goal.as_ref())
    }
}
