// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const RATE_OF_PRODUCTION: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as u32;
    let speed_of_production = (speed * RATE_OF_PRODUCTION) as f64;
    speed_of_production * rate_of_failure_for(speed)
}

fn rate_of_failure_for(speed: u32) -> f64 {
    return match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77, 
        _ => panic!()
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
