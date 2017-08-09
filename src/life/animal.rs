use std;

use life::{Drinkable, Eatable, Living, Mission, MissionResult};
use physics::liquid::Liquid;
use std::rc::Rc;
use tcod::pathfinding::*;
use utils::{Point2D, Point3D, clamp, distance, distance3D,
            nearest_perimeter_point};
use worldgen::WorldState;
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
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Herbivore {
    Cow,
    Sheep,
    Fish,
    Whale,
}

/// Possible animal species.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Species {
    Carnivore(Carnivore),
    Herbivore(Herbivore),
}

/// Represents a limb of an animal.
pub struct BodyPart {
    health: super::HealthLevel,
    name: &'static str,
    missing: bool,
}

/// The animal itself. It keeps track of all its mental and physical states, as well as its goals.
pub struct Animal<'a> {
    thirst: i32,
    hunger: i32,
    goals: Vec<super::Mission>,
    path: Option<AStar<'a>>,
    health: i32,
    arrived: bool,
    failed_goal: Option<super::Mission>,
    pub pos: (usize, usize, usize),
    pub chr: char,
    pub body: [BodyPart; 4],
    pub species: Species,
    pub sight_range: u8,
    pub mood: super::Mood,
    pub current_goal: Option<super::Mission>,
}

impl<'a> Animal<'a> {
    fn satisfy_current_goal(&mut self,
                            ws: &WorldState)
        -> MissionResult {
        let r = self.sight_range as i32;
        let mut adj = vec![];
        let map = ws.map.as_ref().unwrap();
        for x in (-r)..r {
            for y in (-r)..r {
                let (x, y) = (x as usize, y as usize);
                if x * x + y * y <= (r * r) as usize {
                    let maybe_unit = map.get((x, y));
                    adj.push(if let Some(unit) = maybe_unit {
                        let tiles = unit.tiles.borrow();
                        (tiles.get(self.pos.2)
                           .unwrap_or(tiles.get(tiles.len() - 1)
                                           .unwrap_or(&Tile::Empty)).clone(),
                         (x, y, std::cmp::min(self.pos.2, tiles.len())))
                    } else {
                        (Tile::Empty, (x, y, 0))
                    })
                }
            }
        }
        self.step_to_goal(ws, adj)
    }

    fn step_to_goal(&mut self,
                    ws: &WorldState,
                    adj: Vec<(Tile, Point3D)>)
        -> MissionResult {
        self.hunger += 1;
        self.thirst += 1;
        use self::Mission::*;
        if self.path.is_none() && !self.arrived {
            for (tile, pnt) in adj {
                let m = self.current_goal.clone().unwrap();
                match m {
                    Eat(_) => {
                        let carnivore_food = matches!(self.species, Species::Carnivore(..)) &&
                                          matches!(tile, Tile::Item(Item::Food(Food::Meat(..))));
                        let herbivore_food = matches!(self.species, Species::Herbivore(..)) &&
                                          matches!(tile, Tile::Item(Item::Food(Food::Herb(..))));
                        if carnivore_food || herbivore_food {
                            self.create_path_to(ws, pnt);
                            return MissionResult::NoResult;
                        }
                    }
                    Drink(_) => {
                        if matches!(tile, Tile::Item(Item::Food(Food::Water(..)))) {
                            self.create_path_to(ws, pnt);
                            return MissionResult::NoResult;
                        }
                    }
                    AttackEnemy(_) => {
                        for (i, l) in ws.life.iter().enumerate() {
                            let pos = l.borrow().current_pos();
                            let dist = distance((pos.0, pos.1),
                                                (self.pos.0,
                                                 self.pos.1));
                            if pos.2 == self.pos.2 &&
                                dist <= self.sight_range as f32
                            {
                                self.create_path_to(ws, pos);
                                return MissionResult::NoResult;
                            }
                        }
                    }
                    GoToArea(rect, _) => {
                        let p = self.pos.clone();
                        self.create_path_to(ws,
                                            nearest_perimeter_point(rect, p));
                        return MissionResult::NoResult;
                    }
                    ref q => {
                        println!("{:?}", q);
                    }
                }
            }
            self.failed_goal = self.current_goal.clone();
            self.current_goal = None;
        } else if self.path.is_some() {
            self.continue_movement(ws);
        } else if self.arrived {
            return self.stationary_action(ws, adj);
        }
        MissionResult::NoResult
    }

    fn create_path_to(&mut self, ws: &WorldState, pnt: Point3D) {
        let m = ws.map.as_ref().unwrap();
        let (width, height) = m.map_size;
        let mut astar = AStar::new_from_callback(width as i32,
                                                 height as i32,
                                                 ws.can_move(self),
                                                 1.0);
        if astar.find((self.pos.0 as i32, self.pos.1 as i32),
                      (pnt.0 as i32, pnt.1 as i32))
        {
            self.path = Some(astar);
        }
    }

    fn continue_movement(&mut self, ws: &WorldState) {
        let path = self.path.as_mut().unwrap();
        if let Some(npos) = path.walk_one_step(true) {
            self.pos = (npos.0 as usize,
                        npos.1 as usize,
                        ws.location_z((npos.0 as usize,
                                       npos.1 as usize)));
        } else {
            self.arrived = true;
        }
    }

    fn stationary_action(&mut self,
                         ws: &WorldState,
                         adj: Vec<(Tile, Point3D)>)
        -> MissionResult {
        let result = match self.current_goal {
            Some(Mission::AttackEnemy(p)) => {
                for (i, enemy) in ws.life.iter().enumerate() {
                    let pos = enemy.borrow().current_pos();
                    if distance3D(pos, self.pos) <= 2f32 &&
                        self.path
                            .as_mut()
                            .unwrap()
                            .find((self.pos.0 as i32,
                                   self.pos.1 as i32),
                                  (pos.0 as i32, pos.1 as i32))
                    {
                        return MissionResult::Kill(i);
                    }
                }
                MissionResult::NoResult
            }
            _ => MissionResult::NoResult,
        };

        self.current_goal = self.goals.pop();
        result
    }

    fn tolerance(&self) -> i32 { 800 }
}

impl<'a> Living for Animal<'a> {
    fn add_goal(&mut self, mission: Mission) {
        if matches!(mission, Mission::Die) {
            self.failed_goal = None;
            self.current_goal = Some(Mission::Die);
            self.health = 0;
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

    fn execute_mission(&mut self, ws: &WorldState) -> MissionResult {
        if self.current_goal.is_some() {
            self.satisfy_current_goal(ws)
        } else {
            let m = self.goals.pop();
            if m.is_some() {
                self.current_goal = m;
            } else {
                self.auto_add_mission(ws);
            }
            MissionResult::NoResult
        }
    }

    fn auto_add_mission(&mut self,
                        ws: &WorldState)
        -> Option<Mission> {
        if self.goals.len() == 0 {
            if self.thirst >= THIRST_THRESHOLD ||
                self.hunger >= HUNGER_THRESHOLD
            {
                return Some(Mission::Die);
            } else if self.thirst >=
                       THIRST_THRESHOLD - self.tolerance()
            {
                let magnitude = (self.thirst - THIRST_THRESHOLD)
                    .abs() as usize;
                self.add_goal(Mission::Drink(magnitude));
            } else if self.hunger >=
                       HUNGER_THRESHOLD - self.tolerance()
            {
                let magnitude = (self.hunger - THIRST_THRESHOLD)
                    .abs() as usize;
                self.add_goal(Mission::Eat(magnitude));
            } else if let Some(g) = self.failed_goal.clone() {
                match g {
                    Mission::Eat(p) => {
                        self.add_goal(Mission::AttackEnemy(p));
                    }
                    _ => {
                        self.current_goal = self.goals.pop();
                        self.failed_goal = None;
                    }
                }
            }
            self.current_goal.clone()
        } else {
            self.current_goal = self.goals.pop();
            self.current_goal.clone()
        }
    }

    fn current_pos(&self) -> Point3D { self.pos }
}
