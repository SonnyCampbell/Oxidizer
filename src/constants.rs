use strum_macros::{EnumCount, EnumIter};
use strum::{EnumCount, IntoEnumIterator};

use crate::wavetype::WaveType;

pub const SAMPLE_RATE: f32 = 44100.0;
pub const NUM_CHANNELS: u16 = 1;


#[derive(EnumCount, EnumIter, Copy, Clone)]
pub enum OscNumber {
    Osc1,
    Osc2,
    Osc3,
}

#[derive(Default, Clone)]
pub struct LfoParams {
    pub enabled: bool,
    pub wave_type: WaveType,
    pub frequency: f32
}

#[derive(Clone)]
pub struct SoundGenOscParams {
    pub num: OscNumber,
    pub enabled: bool,
    pub wave_type: WaveType,
    pub unisons: i32,
    pub unison_detune_pct: f32
}

impl SoundGenOscParams {
    pub fn create_default_array() -> [SoundGenOscParams; OscNumber::COUNT]  {
        let mut sound_gen_oscillators: Vec<SoundGenOscParams> = Vec::new();
        for osc_num in OscNumber::iter(){
            let i = osc_num as usize;
            let osc = SoundGenOscParams {
                num: osc_num,
                enabled: i == 0,
                wave_type: WaveType::default(),
                unisons: 2,
                unison_detune_pct: 1.0
            };

            sound_gen_oscillators.push(osc);
        }

        return sound_gen_oscillators
                    .try_into()
                    .unwrap_or_else(|v: Vec<SoundGenOscParams>| panic!("Expected a Vec of length {} but it was {}", OscNumber::COUNT, v.len()));

    }
}