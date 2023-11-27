use crate::time;
use crate::oscillator::Oscillator;
use crate::wavetype::WaveType;

pub struct NoteGenerator {
    pub trigger_on_time: f32,
    pub trigger_off_time: f32,
    pub note_pressed: bool,
    
    pub oscillator: Oscillator
}

impl NoteGenerator {
    pub fn new(frequency: f32, sample_rate: f32, wave_type: WaveType) -> NoteGenerator {

        return NoteGenerator{
            trigger_on_time: time::get_time(),
            trigger_off_time: 0.0,
            note_pressed: true,

            oscillator: Oscillator::new(frequency, sample_rate, wave_type),
        };
    }

    pub fn note_released(&mut self){
        self.trigger_off_time = time::get_time();
        self.note_pressed = false;
    }

    pub fn get_sample(&mut self, lfo_freq: f32, lfo_amplitude: f32) -> f32 {
        return self.oscillator.get_sample(lfo_freq, lfo_amplitude);
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType){
        self.oscillator.set_wave_type(wave_type);
    }
    
}