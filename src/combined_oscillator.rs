use rodio::Source;
use std::time::Duration;

use crate::wavetable_oscillator::WavetableOscillator;

pub struct CombinedOscillator {
    oscillators: Vec<WavetableOscillator>
}

impl CombinedOscillator {
    pub fn new() -> CombinedOscillator {
        return CombinedOscillator{
            oscillators: Vec::with_capacity(8)
        };
    }

    pub fn add_oscillator(&mut self, oscillator: WavetableOscillator){
        self.oscillators.push(oscillator);
    }

    fn get_sample(&mut self) -> f32 {
        let mut total = 0.0;

        for mut i in &mut self.oscillators {
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

impl Source for CombinedOscillator{
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        if let Some(table) = self.oscillators.iter().nth(0) {
            return table.sample_rate;
        }

        return 0;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}