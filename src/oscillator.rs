use std::f32::consts::PI;

use crate::wavetype::WaveType;

pub struct Oscillator{
    gain: f32,
    amplitude: f32,
    frequency: f32,
    wave_type: WaveType,

    sample_index: f32,
    sample_rate: f32
}

impl Oscillator {
    pub fn new(freq: f32, sample_rate: f32, wave_type: WaveType) -> Oscillator {
        let gain = 0.0;

        return Oscillator { 
            frequency: freq, 
            gain: gain,
            amplitude: Self::calculate_amplitude(gain),
            sample_index: 1.0,
            sample_rate: sample_rate,
            wave_type: wave_type //TODO make a reference to the value on Synth??
        };
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
        self.sample_index / self.sample_rate
    }

    fn w(&self) -> f32 {
        self.frequency * 2.0 * PI
    }

    fn get_sin_value(&self) -> f32 {
        (self.w() * self.t()).sin()
    }

    fn get_saw_value(&self) -> f32 {
        (2.0 / PI) * (self.frequency * PI * (self.t() % (1.0 / self.frequency)) - (PI / 2.0))
    }

    fn get_tri_value(&self) -> f32 {
        (self.w() * self.t()).sin().asin() * (2.0 / PI)
    }

    fn get_sqr_value(&self) -> f32 {
        if self.get_sin_value() < 0.0 {
            return 1.0;
        }
        
        return -1.0
    }

    fn get_pulse_value(&self) -> f32 {
        let duty_cyle = 0.2;
        if self.get_sin_value() % 1.0 < duty_cyle {
            return 1.0;
        }
        
        return -1.0

    }

    pub fn get_sample(&mut self) -> f32 {

        let sample = match self.wave_type {
            WaveType::Sin => self.get_sin_value(),
            WaveType::Saw => self.get_saw_value(),
            WaveType::Triangle => self.get_tri_value(),
            WaveType::Square => self.get_sqr_value(),
            WaveType::Pulse => self.get_pulse_value(),
        };

        self.sample_index += 1.0;

        return sample * self.amplitude;
    }
}
