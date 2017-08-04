use physics::liquid::Container;
use std::option::Option;
use worldgen::{World, WorldState};

pub mod animal;
pub mod bird;
pub mod dwarf;
pub mod monster;

pub type Priority = usize;

pub type HealthLevel = usize;

#[derive(Debug)]
pub enum Mood {
    Joyful,
    Happy,
    Contented,
    Discontented,
    Unhappy,
    Depressed,
}

// Player assigned missions (orders)
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Order {
    Mine((i32, i32), (i32, i32)),
    GatherPlants((i32, i32), (i32, i32)),
    FellTrees((i32, i32), (i32, i32)),
    CartGoods((i32, i32), (i32, i32), (i32, i32)),
    BuildWall(Vec<(i32, i32)>),
    BuildRoof(Vec<(i32, i32)>),
    Go(i32, i32, i32),
}

pub trait Eatable {
    fn cook(self) -> Self;
    fn neutrition(self) -> Self;
}

pub trait Drinkable {
    fn filter(self) -> Self;
    fn boild(self) -> Self;
    fn satisfaction(self) -> Self;
}

// Basic missions that animals can assign to themselves
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Mission {
    Eat(Priority),
    Drink(Priority),
    AttackEnemy(Priority),
    GoToArea(Priority),
    Obey(Priority, Order),
}
impl Mission {
    fn tag_equals(&self, tag: &Mission) -> bool {
        use self::Mission::*;
        return match (self, tag) {
            (&Eat(..), &Eat(..)) => true,
            (&Drink(..), &Drink(..)) => true,
            (&AttackEnemy(..), &AttackEnemy(..)) => true,
            (&GoToArea(..), &GoToArea(..)) => true,
            (&Obey(..), &Obey(..)) => true,
        };
    }
}

pub trait Living {
    // Adds mission to list of goals
    fn add_goal(&mut self, mission: Mission);
    // Removes mission from list of goals
    fn remove_goal(&mut self, mission: &Mission) -> Mission;
    // Removes n least important missions from goals.
    // Returns removed missions.
    fn prioritize(&mut self, number: usize) -> Vec<Mission>;
    // Chooses highest priority mission, excecutes one step of it, and
    // returns it if done, otherwise returns None.
    fn execute_mission(&self) -> Option<Mission>;
    // Adds a mission when none is provided. Used all the time for
    // animals. If there is already a mission going, returns None.
    fn auto_add_mission(&self) -> Option<Mission>;
}
