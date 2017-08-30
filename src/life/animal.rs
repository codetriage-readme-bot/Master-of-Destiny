use std;

use draw::DrawChar;
use life::{Living, Mission, MissionResult};
use tcod::Console;
use tcod::RootConsole;
use tcod::pathfinding::*;
use utils::{Point3D, can_move, distance, nearest_perimeter_point,
            random_point};
use worldgen::World;
use worldgen::terrain::{Food, Item, Tile};

const THIRST_THRESHOLD: i32 = 259_200;
const HUNGER_THRESHOLD: i32 = 1_814_400;

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
    path: Option<AStarIterator>,
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
                     hunger: 50,
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
                        println!("    Looking at ({}, {})", x, y);
                        let tiles = unit.tiles.borrow();
                        (tiles.get(self.pos.2)
                           .unwrap_or(tiles.get(tiles.len() - 1)
                                           .unwrap_or(&Tile::Empty)).clone(),
                         (x as usize, y as usize, std::cmp::min(self.pos.2, tiles.len())))
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
        self.hunger += 1;
        self.thirst += 1;
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
                                println!("      Edible food unit found");
                                if !self.create_path_to(map, pnt) {
                                    println!("      Path to goal not found");
                                    self.current_goal = None;
                                    self.failed_goal = Some(m);
                                }
                                return MissionResult::NoResult;
                            }
                        }
                        Drink(_) => {
                            if matches!(tile, Tile::Item(Item::Food(Food::Water(..)))) {
                                println!("      Drinkable liquid found");
                                if !self.create_path_to(map, pnt) {
                                    println!("      Path to goal not found");
                                    self.current_goal = None;
                                    self.failed_goal = Some(m);
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
                                    println!("      Enemy in sight, persuing");
                                    if !self.create_path_to(map,
                                                            pnt)
                                    {
                                        println!("      Cannot find path to enemy");
                                        self.current_goal = None;
                                        self.failed_goal = Some(m);
                                    }
                                    return MissionResult::NoResult;
                                }
                            }
                        }
                        GoToArea(rect, _) => {
                            let p = self.pos.clone();
                            if !self.create_path_to(map,
                                                    nearest_perimeter_point(rect, p)) {
                                println!("      Cannot find path to area.");
                                self.current_goal = None;
                                self.failed_goal = Some(m);
                            }
                            return MissionResult::NoResult;
                        }
                        Go(point, _) => {
                            let p = self.pos.clone();
                            if !self.create_path_to(map, (point.0, point.1, map.location_z(point))) {
                                println!("       Cannot find path to point {:?}", point);
                                self.current_goal = None;
                                self.failed_goal = Some(m);
                            }
                            return MissionResult::NoResult;
                        }
                        ref q => {}
                    }
                } else {
                    println!("Adding nonexistant mission");
                    self.auto_add_mission(map);
                }
            }
            self.failed_goal = self.current_goal.clone();
            self.current_goal = None;
        } else if self.path.is_some() {
            println!("Moving along path");
            self.continue_movement(map);
        } else if self.arrived {
            println!("Performing action on path destination");
            return self.stationary_action(map, adj);
        }
        MissionResult::NoResult
    }

    fn create_path_to(&mut self, map: &World, pnt: Point3D) -> bool {
        let (width, height) = map.map_size;
        let mut astar = AStar::new_from_callback(width as i32,
                                                 height as i32,
                                                 can_move(map),
                                                 1.0);
        if astar.find((self.pos.0 as i32, self.pos.1 as i32),
                      (pnt.0 as i32, pnt.1 as i32))
        {
            self.path = Some(astar.walk());
            true
        } else {
            false
        }
    }

    fn continue_movement(&mut self, map: &World) {
        let path = self.path.as_mut().unwrap();
        if let Some(npos) = path.next() {
            self.pos = (npos.0 as usize,
                        npos.1 as usize,
                        map.location_z((npos.0 as usize,
                                        npos.1 as usize)));
        } else {
            self.arrived = true;
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
                    adj.iter().find(|&&(t, _)| {
                        matches!(t, Tile::Item(Item::Food(..)))
                    }) {
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
        self.goals.drain(0..n).collect()
    }

    fn execute_mission(&mut self, map: &World) -> MissionResult {
        if self.current_goal.is_some() {
            println!("Executing goal: {:?}",
                     self.current_goal.clone().unwrap());
            self.satisfy_current_goal(map)
        } else {
            println!("Need new goal:");
            let m = self.goals.pop();
            println!("  Candidate goal: {:?}", m);
            if m.is_some() {
                println!("    New goal: {:?}", m);
                self.current_goal = m;
            } else {
                println!("    No goals, adding.");
                self.auto_add_mission(map);
            }
            self.satisfy_current_goal(map)
        }
    }

    fn auto_add_mission(&mut self, map: &World) -> Option<Mission> {
        if self.goals.len() == 0 {
            if self.thirst >= THIRST_THRESHOLD ||
                self.hunger >= HUNGER_THRESHOLD
            {
                return Some(Mission::Die);
            } else if self.thirst >=
                       THIRST_THRESHOLD - self.tolerance()
            {
                self.prioritize(2);
                let magnitude = (self.thirst - THIRST_THRESHOLD)
                    .abs() as usize;
                self.add_goal(Mission::Drink(magnitude));
            } else if self.hunger >=
                       HUNGER_THRESHOLD - self.tolerance()
            {
                self.prioritize(2);
                let magnitude = (self.hunger - THIRST_THRESHOLD)
                    .abs() as usize;
                self.add_goal(Mission::Eat(magnitude));
            } else if let Some(g) = self.failed_goal.clone() {
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
            } else {
                self.current_goal =
                    Some(Mission::Go(random_point(0,
                                                  map.map_size.0,
                                                  0,
                                                  map.map_size.1),
                                     10));
            }
            self.current_goal.clone()
        } else {
            self.current_goal = self.goals.pop();
            self.current_goal.clone()
        }
    }

    fn current_pos(&self) -> Point3D { self.pos }
    fn species(&self) -> &SpeciesProperties { &self.species }
}
