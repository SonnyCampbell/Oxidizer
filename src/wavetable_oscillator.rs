
use rodio::Source;
use std::time::Duration;

pub struct WavetableOscillator{
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    gain: f32,
    amplitude: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        let gain = 0.0;

        return WavetableOscillator { 
            sample_rate: sample_rate, 
            wave_table: wave_table, 
            index: 0.0, 
            index_increment: 0.0,
            gain: gain,
            amplitude: Self::calculate_amplitude(gain)
        };
    }

    fn calculate_amplitude(gain: f32) -> f32 {
        return (10.0 as f32).powf(gain / 20.0);
    }

    pub fn set_frequency(&mut self, frequency: f32){
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn set_gain(&mut self, gain: f32){
        self.gain = gain;
        self.amplitude = Self::calculate_amplitude(gain);
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample * self.amplitude;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        // if self.index is 1.25 we do
        // wave_table[truncated_index] * 0.75 + wave_table[next_index] * 0.25
        // because the index is nearer to the truncated_index than the next_index

        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32>{
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator{
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}
