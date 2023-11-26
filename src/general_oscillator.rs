use crate::time;
use crate::oscillator::Oscillator;
use crate::wavetype::WaveType;

pub struct GeneralOscillator {
    note_oscillator: Oscillator,
    pub trigger_on_time: f32,
    pub trigger_off_time: f32,
    pub note_pressed: bool
}

impl GeneralOscillator {
    pub fn new(freq: f32, sample_rate: f32, wave_type: WaveType) -> GeneralOscillator {
        return GeneralOscillator {
            note_oscillator: Oscillator::new(freq, sample_rate, wave_type),
            trigger_on_time: time::get_time(),
            trigger_off_time: 0.0,
            note_pressed: true,
        };
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType){
        self.note_oscillator.set_wave_type(wave_type);
    }

    pub fn note_released(&mut self){
        self.trigger_off_time = time::get_time();
        self.note_pressed = false;
    }

    pub fn get_sample(&mut self) -> f32 {
        return self.note_oscillator.get_sample();
    }
}