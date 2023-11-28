pub static SAMPLE_RATE: f32 = 44100.0;
pub static NUM_CHANNELS: u16 = 1;

use strum_macros::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter)]
pub enum OscNumber {
    Osc1,
    Osc2,
    Osc3,
}