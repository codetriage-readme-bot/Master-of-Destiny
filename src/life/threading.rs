extern crate bichannels;
use self::bichannels::Endpoint;

use super::Living;
use life::{DrawableLiving, MissionResult};
use life::animal::{Species, SpeciesProperties};
use std::cell::RefCell;
use std::ops::Index;
use utils::{Point3D, distance3_d};
use worldgen::AnimalHandlerEvent;

pub struct LifeManager {
    life: Vec<RefCell<Box<Living>>>,
}

impl Index<usize> for LifeManager {
    type Output = RefCell<Box<Living>>;
    fn index(&self, location: usize) -> &Self::Output {
        &self.life[location]
    }
}

impl LifeManager {
    pub fn new() -> Self { LifeManager { life: vec![] } }

    pub fn posns_of_species(&self, s: Species) -> Vec<Point3D> {
        self.life
            .iter()
            .filter_map(|a| if let Ok(l) = a.try_borrow() {
                            if l.species().species == s {
                                Some(l.current_pos())
                            } else {
                                None
                            }
                        } else {
                            None
                        })
            .collect()
    }

    pub fn closeby(&self,
                   this: (Point3D, SpeciesProperties))
        -> Vec<(Point3D, SpeciesProperties)> {
        let mut res = vec![];
        for l in self.life.iter() {
            let other = l.borrow();
            let dist = distance3_d(other.current_pos(), this.0);
            let os = other.species();
            let ts = this.1;

            if dist <= ts.sight as f32 && os.species != ts.species {
                res.push((other.current_pos(), *os));
            }
        }

        res
    }

    pub fn closeby_predator(&self,
                            this: (Point3D, SpeciesProperties))
        -> Option<SpeciesProperties> {
        self.closeby(this)
            .iter()
            .find(|a| matches!(a.1.species, Species::Carnivore(..)))
            .map(|a| a.1)
    }

    pub fn kill(&mut self, i: usize) -> Box<Living> {
        self.life.remove(i).into_inner()
    }

    pub fn len(&self) -> usize { self.life.len() }

    pub fn life_at_point(
        &self,
        x: usize,
        y: usize)
        -> Option<(usize, &RefCell<Box<Living>>)> {
        self.life
            .iter()
            .enumerate()
            .find(|&(_i, e)| {
            let op = e.borrow().current_pos();
            (op.0, op.1) == (x, y)
        })
    }

    pub fn get_drawables(&self) -> Vec<DrawableLiving> {
        self.life
            .iter()
            .map(|l| {
            let l = l.borrow();
            DrawableLiving {
                species: *l.species(),
                current_pos: l.current_pos(),
            }
        })
            .collect()
    }
}

pub fn handle_life(world_endp: Endpoint<MissionResult,
                                        AnimalHandlerEvent>)
    -> impl FnOnce() {
    move || {
        let mut life = LifeManager::new();
        match world_endp.recv() {
            Ok(AnimalHandlerEvent::NewAnimal(animal)) => {
                life.life.push(RefCell::new(animal));
            }
            Ok(AnimalHandlerEvent::Draw) => {
                world_endp.send_fail(MissionResult::List(life.get_drawables()));
            }
            Ok(AnimalHandlerEvent::Update(time, world)) => {
                let mut many = vec![];
                for i in 0..life.len() {
                    let modifier = if i % 2 == 0 { 2 } else { 3 };
                    if time % modifier == 0 {
                        let res = {
                            let mut actor = life[i].borrow_mut();
                            actor.execute_mission(&world, &life)
                        };
                        many.push(match res {
                            MissionResult::Die(j) |
                            MissionResult::Kill(j) => {
                                let l = life.kill(if j != 0 {
                                                      j
                                                  } else {
                                                      i
                                                  });
                                MissionResult::Kill2(l.current_pos(),
                                                     l.species()
                                                      .species)
                            }
                            other => other,
                        });
                    }
                }
                world_endp.send_fail(MissionResult::Many(many));
            }
            _ => (),
        }
    }
}
