use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

pub fn get_time() -> f32 {
    START_TIME.elapsed().as_secs_f32()
}