use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum Time {
    Night2,
    Morning,
    Noon,
    Afternoon,
    Night1,
    Midnight,
}

/*
Step: 5
Total: 100

|-- Midnight
|
|
|-- Night2
|
|
|-- Morning
|
|
|
|-- Noon
|
|
|
|-- Afternoon
|
|
|
|-- Night1
|
|
 */
pub fn calculate_time_of_day(time: usize,
                             cycle_length: usize)
                             -> Time {
    let percent = time as f32 / cycle_length as f32;

    if percent == 0.0 || percent >= 95.0 {
        Time::Midnight
    } else if percent <= 20.0 {
        Time::Night2
    } else if percent <= 55.0 {
        Time::Morning
    } else if percent <= 55.0 {
        Time::Noon
    } else if percent <= 75.0 {
        Time::Afternoon
    } else if percent <= 95.0 {
        Time::Night1
    } else {
        panic!("Nonsense percentage, {}", percent)
    }
}

pub fn get_world_time() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}
