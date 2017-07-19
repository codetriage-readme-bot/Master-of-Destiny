use std::collections::HashMap;
use std::rc::Rc;
use worldgen::Unit;
use worldgen::terrain::*;

pub fn solid_physics(aj: Vec<Rc<Unit>>)
                     -> Option<HashMap<(i32, i32), Unit>> {
    None
}
pub fn liquid_physics(aj: Vec<Rc<Unit>>)
                      -> Option<HashMap<(i32, i32), Unit>> {
    None
}
