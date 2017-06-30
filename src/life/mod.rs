use std::option::Option;
use worldgen::{World, WorldState};

pub mod animal;
pub mod bird;
pub mod dwarf;
pub mod monster;

pub trait Animal {
    fn act<A: Animal>(&mut self, WorldState<A>) -> bool;
    fn attack<T: Animal>(&mut self, &mut T) -> Option<T>;
    fn eat(&mut self, pos: (i32, i32)) -> bool;
    fn drink(&mut self, pos: (i32, i32)) -> bool;
    fn pos(&self) -> (i32, i32);
}
