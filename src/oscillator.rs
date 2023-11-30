use std::f32::consts::PI;

use crate::constants::*;
use crate::wavetype::WaveType;

pub struct Oscillator{
    gain: f32,
    amplitude: f32,
    frequency: f32,
    wave_type: WaveType,

    sample_index: f32,
}

impl Oscillator {
    pub fn new(frequency: f32, wave_type: WaveType, starting_sample_index: f32) -> Oscillator {
        let gain = 0.0;

        return Oscillator { 
            gain: gain,
            frequency, 
            amplitude: Self::calculate_amplitude(gain),
            sample_index: starting_sample_index,
            wave_type: wave_type, //TODO could I make a reference to the value on Synth?? Lifetime questions...

        };
    }

    pub fn get_sample_index(&self) -> f32 {
        return self.sample_index;
    }

    pub fn get_frequency(&self) -> f32 {
        return self.frequency;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn get_amplitude(&self) -> f32 {
        return self.amplitude;
    }

    fn calculate_amplitude(gain: f32) -> f32 {
        return (10.0 as f32).powf(gain / 20.0);
    }

    pub fn set_gain(&mut self, gain: f32){
        self.gain = gain;
        self.amplitude = Self::calculate_amplitude(gain);
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType){
        self.wave_type = wave_type;
    }

    fn t(&self) -> f32 {
        self.sample_index / SAMPLE_RATE
    }

    fn w(&self, frequency: f32) -> f32 {
        frequency * 2.0 * PI
    }

    fn get_sin_value(&self, frequency: f32) -> f32 {
        frequency.sin()
    }

    fn get_saw_value(&self, frequency: f32) -> f32 {
        let freq_in_hz = frequency / (2.0 * PI * self.t());

        (2.0 / PI) * (freq_in_hz * PI * (self.t() % (1.0 / freq_in_hz)) - (PI / 2.0))
    }

    fn get_tri_value(&self, frequency: f32) -> f32 {
        self.get_sin_value(frequency).asin() * (2.0 / PI)
    }

    fn get_sqr_value(&self, frequency: f32) -> f32 {
        if self.get_sin_value(frequency) < 0.0 {
            return 1.0;
        }
        
        return -1.0
    }

    fn get_pulse_value(&self, frequency: f32) -> f32 {
        let duty_cyle = 0.2;
        if self.get_sin_value(frequency) % 1.0 < duty_cyle {
            return 1.0;
        }
        
        return -1.0
    }

    fn get_modulated_freq(&self, lfo_freq: f32, lfo_amplitude: f32) -> f32{
        let base_freq = self.w(self.frequency) * self.t();
        let lfo_part = lfo_amplitude * self.frequency * (self.w(lfo_freq) * self.t()).sin();

        return base_freq + lfo_part;
    }

    pub fn get_sample(&mut self, lfo_freq: f32, lfo_amplitude: f32) -> f32 {

        let modulated_freq = self.get_modulated_freq(lfo_freq, lfo_amplitude);

        let sample = match self.wave_type {
            WaveType::Sin => self.get_sin_value(modulated_freq),
            WaveType::Saw => self.get_saw_value(modulated_freq),
            WaveType::Triangle => self.get_tri_value(modulated_freq),
            WaveType::Square => self.get_sqr_value(modulated_freq),
            WaveType::Pulse => self.get_pulse_value(modulated_freq),
        };

        self.sample_index += 1.0;

        return sample * self.amplitude;
    }
}
