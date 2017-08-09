use std;

use life::{Drinkable, Eatable, Living, Mission};
use physics::liquid::Liquid;
use std::rc::Rc;
use tcod::pathfinding::*;
use utils::{Point2D, Point3D, clamp, distance,
            nearest_perimeter_point};
use worldgen::WorldState;
use worldgen::terrain::{Food, Item, Tile};

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
    prey: Rc<Living>,
    arrived: bool,
    pub pos: (usize, usize, usize),
    pub chr: char,
    pub body: [BodyPart; 4],
    pub species: Species,
    pub sight_range: u8,
    pub mood: super::Mood,
    pub current_goal: Option<super::Mission>,
}

impl<'a> Animal<'a> {
    fn satisfy_current_goal(&mut self, ws: &WorldState) {
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
        self.step_to_goal(ws, adj);
    }

    fn step_to_goal(&mut self,
                    ws: &WorldState,
                    adj: Vec<(Tile, Point3D)>) {
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
                            return;
                        }
                    }
                    Drink(_) => {
                        if matches!(tile, Tile::Item(Item::Food(Food::Water(..)))) {
                            self.create_path_to(ws, pnt);
                            return;
                        }
                    }
                    AttackEnemy(_) => {
                        for (i, l) in ws.life.iter().enumerate() {
                            let pos = l.current_pos();
                            let dist = distance((pos.0, pos.1),
                                                (self.pos.0,
                                                 self.pos.1));
                            if pos.2 == self.pos.2 &&
                                dist <= self.sight_range as f32
                            {
                                self.create_path_to(ws, pos);
                                self.prey = ws.life[i].clone();
                                return;
                            }
                        }
                    }
                    GoToArea(rect, _) => {
                        let p = self.pos.clone();
                        self.create_path_to(ws,
                                            nearest_perimeter_point(rect, p));
                        return;
                    }
                    ref q => {
                        println!("{:?}", q);
                    }
                }
            }
        } else if self.path.is_some() {
            self.continue_movement(ws);
        } else if self.arrived {
            self.stationary_action(ws, adj);
        }
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

    fn stationary_action(&self,
                         ws: &WorldState,
                         adj: Vec<(Tile, Point3D)>) {
    }
}

impl<'a> Living for Animal<'a> {
    fn add_goal(&mut self, mission: Mission) {
        match self.goals.binary_search(&mission) {
            Ok(_) => {}
            Err(i) => self.goals.insert(i, mission),
        };
    }

    fn remove_goal(&mut self, tag: &Mission) -> Option<Mission> {
        self.goals.remove_item(tag)
    }

    fn prioritize(&mut self, n: usize) -> Vec<Mission> {
        self.goals.drain(0..n).collect()
    }

    fn execute_mission(&mut self, ws: &WorldState) {
        if self.current_goal.is_some() {
            self.satisfy_current_goal(ws);
        } else {
            let m = self.goals.pop();
            if m.is_some() {
                self.current_goal = m;
            } else {
                self.auto_add_mission(ws);
            }
        }
    }

    fn auto_add_mission(&self, ws: &WorldState) -> Option<Mission> {
        None
    }

    fn current_pos(&self) -> Point3D { self.pos }
}
