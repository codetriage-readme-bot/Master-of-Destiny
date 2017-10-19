use draw::DrawChar;
use std::option::Option;

use tcod::{BackgroundFlag, Console, RootConsole};

use utils::{Point2D, Point3D, Rect2D, Rect2D3D};
use worldgen::World;
use worldgen::terrain::{Item, Tile};

pub mod animal;
pub mod bird;
pub mod dwarf;
pub mod monster;

pub type Priority = usize;

pub type HealthLevel = usize;

/// The mental mood of a living actor.
#[derive(Debug, Copy, Clone)]
pub enum Mood {
    Angry,
    Fearful,
    Agressive,
    Wary,
    Joyful,
    Happy,
    Contented,
    Discontented,
    Unhappy,
    Depressed,
}

/// Player assigned missions (orders)
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Order {
    GatherPlants(Rect2D),
    FellTrees(Rect2D),

    CartGoods(Rect2D),
    Go(Point3D),

    Mine(Rect2D),
    BuildWall(Rect2D),
    BuildFence(Rect2D),
    BuildRamp(Point2D),
}

/// Actions that can be performed on an eatable object.
pub trait Eatable {
    fn cook(self) -> Self;
}

/// Actions that can be performed on a drinkable object.
pub trait Drinkable {
    fn filter(self) -> Self;
    fn boild(self) -> Self;
    fn satisfaction(self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MissionResult {
    NoResult,
    Die,
    Kill(usize),
    ReplaceItem(Point3D, Item),
    RemoveItem(Point3D),
}

/// Basic missions that animals can assign to themselves
#[derive(Debug, Eq, PartialOrd, Clone, Copy)]
pub enum Mission {
    Eat(Priority),
    PickFood(Priority),
    Drink(Priority),
    AttackEnemy(Priority),
    GoToArea(Rect2D3D, Priority),
    Go(Point2D, Priority),
    Obey(Priority, Order),
    Die,
}

use std::cmp::*;
impl Ord for Mission {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Mission::*;
        let priority_a = match self {
            &PickFood(p) | &Eat(p) | &Drink(p) |
            &AttackEnemy(p) | &GoToArea(_, p) | &Obey(p, _) => p,
            &Go(_, p) => p,
            &Die => 1000,
        };

        let priority_b = match other {
            &PickFood(p) | &Eat(p) | &Drink(p) |
            &AttackEnemy(p) | &GoToArea(_, p) | &Obey(p, _) => p,
            &Go(_, p) => p,
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
    /// Returns the current goal.
    fn goals(&self) -> (Option<&Mission>, Option<&Mission>);
    /// Chooses highest priority mission, excecutes one step of it, and
    /// returns it if done, otherwise returns None.
    fn execute_mission(&mut self, ws: &World) -> MissionResult;
    /// Adds a mission when none is provided. Used all the time for
    /// animals. If there is already a mission going, returns None.
    fn auto_add_mission(&mut self,
                        ws: &World,
                        adj: Vec<(Tile, Point3D)>)
        -> Option<Mission>;

    fn current_goal(&self) -> Option<Mission>;
    fn current_pos(&self) -> (usize, usize, usize);
    fn species(&self) -> &animal::SpeciesProperties;
}
impl DrawChar for Living {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize)) {
        root.put_char(pos.0 as i32,
                      pos.1 as i32,
                      self.species().chr,
                      BackgroundFlag::Default);
    }
}
