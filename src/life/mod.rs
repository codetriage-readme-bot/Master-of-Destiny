use std::option::Option;
use utils::{Point2D, Point3D};
use worldgen::WorldState;
use worldgen::terrain::Item;

pub mod animal;
pub mod bird;
pub mod dwarf;
pub mod monster;

pub type Priority = usize;

pub type HealthLevel = usize;

/// The mental mood of a living actor.
#[derive(Debug, Copy, Clone)]
pub enum Mood {
    Joyful,
    Happy,
    Contented,
    Discontented,
    Unhappy,
    Depressed,
}

/// Player assigned missions (orders)
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Order {
    Mine(Point2D, Point2D),
    GatherPlants(Point2D, Point2D),
    FellTrees((i32, i32), Point2D),
    CartGoods(Point2D, Point2D, Point2D),
    BuildWall(Vec<Point2D>),
    BuildRoof(Vec<Point2D>),
    Go(Point3D),
}

/// Actions that can be performed on an eatable object.
pub trait Eatable {
    fn cook(self) -> Self;
    fn neutrition(self) -> Self;
}

/// Actions that can be performed on a drinkable object.
pub trait Drinkable {
    fn filter(self) -> Self;
    fn boild(self) -> Self;
    fn satisfaction(self) -> Self;
}

#[derive(Debug, Clone)]
pub enum MissionResult {
    NoResult,
    Kill(usize),
    ReplaceItem(Point3D, Item),
    RemoveItem(Point3D),
}

/// Basic missions that animals can assign to themselves
#[derive(Debug, Eq, PartialOrd, Clone)]
pub enum Mission {
    Eat(Priority),
    Drink(Priority),
    AttackEnemy(Priority),
    GoToArea(((usize, usize), (usize, usize), usize), Priority),
    Obey(Priority, Order),
    Die,
}

use std::cmp::*;
impl Ord for Mission {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Mission::*;
        let priority_a = match self {
            &Eat(p) | &Drink(p) | &AttackEnemy(p) |
            &GoToArea(_, p) | &Obey(p, _) => p,
            &Die => 1000,
        };

        let priority_b = match other {
            &Eat(p) | &Drink(p) | &AttackEnemy(p) |
            &GoToArea(_, p) | &Obey(p, _) => p,
            &Die => 1000,
        };

        return priority_a.cmp(&priority_b);
    }
}

impl PartialEq for Mission {
    fn eq(&self, other: &Mission) -> bool {
        use self::Mission::*;
        return match (self, other) {
            (&Eat(..), &Eat(..)) => true,
            (&Drink(..), &Drink(..)) => true,
            (&AttackEnemy(..), &AttackEnemy(..)) => true,
            (&GoToArea(..), &GoToArea(..)) => true,
            (&Obey(..), &Obey(..)) => true,
            _ => false,
        };
    }
}

pub trait Living {
    /// Adds mission to list of goals
    fn add_goal(&mut self, mission: Mission);
    /// Removes mission from list of goals
    fn remove_goal(&mut self, tag: &Mission) -> Option<Mission>;
    /// Removes n least important missions from goals.
    /// Returns removed missions.
    fn prioritize(&mut self, number: usize) -> Vec<Mission>;
    /// Chooses highest priority mission, excecutes one step of it, and
    /// returns it if done, otherwise returns None.
    fn execute_mission(&mut self, ws: &WorldState) -> MissionResult;
    /// Adds a mission when none is provided. Used all the time for
    /// animals. If there is already a mission going, returns None.
    fn auto_add_mission(&mut self) -> Option<Mission>;

    fn current_pos(&self) -> (usize, usize, usize);
}
