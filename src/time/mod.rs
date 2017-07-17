use draw::Describe;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq)]
pub enum Time {
    Night2,
    Morning,
    Noon,
    Afternoon,
    Night1,
    Midnight,
}
impl Time {
    pub fn from_clock_time(clock: &Clock) -> Self {
        let hours = clock.time.0;
        if hours == 0 {
            Time::Midnight
        } else if hours > 0 && hours <= 7 {
            Time::Night2
        } else if hours < 12 {
            Time::Morning
        } else if hours <= 1 {
            Time::Noon
        } else if hours <= 6 {
            Time::Afternoon
        } else {
            Time::Night1
        }
    }
}
impl Describe for Time {
    fn describe(&self) -> String {
        match self {
            &Time::Night2 => "late night".to_string(),
            &Time::Morning => "morning".to_string(),
            &Time::Noon => "noon".to_string(),
            &Time::Afternoon => "afternoon".to_string(),
            &Time::Night1 => "afternoon or early night".to_string(),
            &Time::Midnight => "midnight".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Season {
    Autumn,
    Winter,
    Spring,
    Summer,
}

impl Season {
    pub fn from_year(month: usize) -> Self {
        if 3 <= month && month <= 5 {
            Season::Spring
        } else if 6 <= month && month <= 8 {
            Season::Summer
        } else if 9 <= month && month <= 11 {
            Season::Autumn
        } else {
            Season::Winter
        }
    }
}

pub struct Calendar {
    pub dmy: (usize, usize, usize),
    pub season: Season,
}

impl Describe for Calendar {
    fn describe(&self) -> String {
        format!("{:?}, {}/{}/{}",
                self.season,
                self.dmy.0,
                self.dmy.1,
                self.dmy.2)
    }
}

impl Calendar {
    pub fn update_to_day(&mut self, days: usize) {
        self.dmy = (days % 30, days / 30, days / 356);
        self.season = Season::from_year(self.dmy.2);
    }
}

pub struct Clock {
    pub time: (usize, usize),
}

impl Describe for Clock {
    fn describe(&self) -> String {
        format!("{}:{}", self.time.0, self.time.1)
    }
}

impl Clock {
    pub fn update_deltatime(&mut self, dt: usize) {
        let mins = self.time.1 + dt;
        if mins >= 60 {
            self.time = (self.time.0 + 1, 0);
        } else {
            self.time = (self.time.0, mins);
        }
    }
}

pub fn get_world_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}
