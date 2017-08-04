use life::{Drinkable, Eatable, Living, Mission};
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
    current_goal: Option<super::Mission>,
    goals: Vec<super::Mission>,
    path: pathfinding::AStar<'a>,
}

impl<'a> Living for Animal<'a> {
    fn add_goal(&mut self, mission: Mission) {
        match self.goals.binary_search(&mission) {
            Ok(_) => {}
            Err(i) => self.goals.insert(i, mission),
        };
    }

    fn remove_goal(&mut self, tag: &Mission) -> Mission {
        let item_to_remove =
            self.goals
            .iter()
            .find(|&item| item.tag_equals(tag));
        self.goals.remove_item(item_to_remove);
        item_to_remove
    }

    fn prioritize(&mut self, n: usize) -> Vec<Mission> {
        let least = self.goals.iter().take(n).map(|x| *x);
        self.goals = self.goals
            .iter()
            .skip(n)
            .map(|x| *x)
            .collect();
        least.collect()
    }

    fn execute_mission(&self) -> Option<Mission> {
        if let Some(m) = self.current_goal {
            self.satisfy_goal(m)
        } else {
            let m = self.goals.pop();
            if m.is_some() {
                self.current_goal = m;
                m
            } else {
                self.auto_add_mission()
            }
        }
    }

    fn auto_add_mission(&self) -> Option<Mission> { None }
}
