use rodio::Source;
use std::{time::Duration, env::join_paths};

use crate::general_oscillator::GeneralOscillator;

pub struct CombinedOscillator {
    oscillators: Vec<GeneralOscillator>
}

impl CombinedOscillator {
    pub fn new() -> CombinedOscillator {
        return CombinedOscillator{
            oscillators: Vec::with_capacity(8)
        };
    }

    pub fn add_oscillator(&mut self, oscillator: GeneralOscillator){
        self.oscillators.push(oscillator);
    }

    pub fn len(&self) -> usize {
        return self.oscillators.len();
    }

    fn get_sample(&mut self) -> f32 {
        let mut total = 0.0;

        for i in &mut self.oscillators {
            total += i.get_sample();
        }

        return total;
    }
}

impl Iterator for CombinedOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32>{
        return Some(self.get_sample());
    }
}

impl Source for CombinedOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return 44100; //Todo sample rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}