use std::cell::RefCell;
use std::rc::Rc;
use utils::weak_adjacent;
use worldgen::Unit;
use worldgen::terrain::*;

pub struct Container<L: Liquid<L>> {
    size: i32,
    name: &'static str,
    liquids: Vec<L>,
}

impl<L> Container<L>
    where
    L: Liquid<L>,
{
    fn contents(&self) -> i32 {
        self.liquids
            .iter()
            .fold(0, |sum, x| sum + x.quantity())
    }
    fn is_full(&self) -> bool { self.contents() < self.size }

    fn can_fit(&self, liq: &L) -> bool {
        self.contents() + liq.quantity() < self.size
    }

    fn is_empty(&self) -> bool { self.liquids.len() == 0 }

    fn fill(&mut self, liq: L) -> bool {
        if self.can_fit(&liq) {
            self.liquids.push(liq);
            true
        } else if self.is_full() {
            false
        } else {
            let nliq = liq.new(self.size - self.contents());
            self.fill(nliq)
        }
    }
}

pub trait Liquid<L: Liquid<L>> {
    fn quantity(&self) -> i32;
    fn amount(&self) -> i32;
    fn new(&self, quantity: i32) -> L;
}

pub fn solid_physics(pnt: (usize, usize),
                     aj: Vec<Rc<Unit>>)
                     -> Option<Vec<Unit>> {
    None
}
pub fn liquid_physics(pnt: (usize, usize),
                      aj: Vec<Rc<Unit>>)
                      -> Option<Vec<Unit>> {
    Some(
        aj.iter()
            .map(|u| {
                let mut ut = u.tiles.borrow_mut();
                let len = ut.len();
                let new_height = len % 23;
                if new_height <= len {
                    Unit {
                        biomes: u.biomes.clone(),
                        tiles: RefCell::new(ut.iter()
                                            .take(new_height)
                                            .map(|x| *x)
                                            .collect()),
                    }
                } else {
                    let mut tiles = ut;
                    for x in 0..(new_height - len) {
                        tiles.push(Tile::Water(LiquidPurity::Clean,
                                               State::Liquid,
                                               (len + x) as i32));
                    }
                    Unit {
                        biomes: u.biomes.clone(),
                        tiles: RefCell::new(tiles.to_vec()),
                    }
                }
            })
            .collect(),
    )
}
