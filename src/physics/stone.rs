use std::rc::Rc;
use worldgen::Unit;

pub fn solid_physics(_pnt: (usize, usize),
                     _aj: Vec<Rc<Unit>>)
                     -> Option<Vec<Unit>> {
    None
}
pub fn liquid_physics(_pnt: (usize, usize),
                      _aj: Vec<Rc<Unit>>)
                      -> Option<Vec<Unit>> {
    None
}
