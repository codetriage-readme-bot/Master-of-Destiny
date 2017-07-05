use physics::liquid::Liquid;
use tcod::pathfinding;

pub struct BodyPart {
    health: super::HealthLevel,
    missing: bool,
}

pub struct Animal<'a> {
    pos: (i32, i32),
    chr: char,
    body: [BodyPart; 4],
    hunger: i32,
    thirst: i32,
    mood: super::Mood,
    goals: Vec<super::Mission>,
    path: pathfinding::AStar<'a>,
}
