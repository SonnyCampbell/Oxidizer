use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> f32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f32()
}