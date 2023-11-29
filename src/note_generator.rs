

use strum::EnumCount;

use crate::constants::OscNumber;
use crate::time;
use crate::oscillator::Oscillator;
use crate::wavetype::WaveType;



pub struct NoteGenerator {
    pub trigger_on_time: f32,
    pub trigger_off_time: f32,
    pub note_pressed: bool,
    
    oscillators: [Option<Oscillator>; OscNumber::COUNT]
}

impl NoteGenerator {
    pub fn new(frequency: f32, wave_types: [Option<WaveType>; OscNumber::COUNT]) -> NoteGenerator {
        const INIT: Option<Oscillator> = None;
        let mut oscillators: [Option<Oscillator>; OscNumber::COUNT] = [INIT; OscNumber::COUNT];

        let mut i = 0;
        for opt in wave_types {
            match opt {
                Some(wave_type) => 
                    oscillators[i] = Some(Oscillator::new(frequency, wave_type)),
                None => oscillators[i] = None,
            }

            i += 1;
        }

        return NoteGenerator{
            trigger_on_time: time::get_time(),
            trigger_off_time: 0.0,
            note_pressed: true,
            oscillators: oscillators,
        };
    }

    pub fn note_released(&mut self){
        self.trigger_off_time = time::get_time();
        self.note_pressed = false;
    }

    pub fn get_sample(&mut self, lfo_freq: f32, lfo_amplitude: f32) -> f32 {
        let mut total = 0.0;
        
        for opt in &mut self.oscillators {
            match opt {
                Some(osc) => total += osc.get_sample(lfo_freq, lfo_amplitude),
                None => {},
            } 
        }

        return total;
    }

    pub fn set_wave_type(&mut self, wave_types: [Option<WaveType>; 3]){
        let mut i = 0;
        for opt in wave_types {
            match opt {
                Some(wave_type) => {
                    if let Some(osc) = &mut self.oscillators[i] {
                        osc.set_wave_type(wave_type);
                    }
                },
                None => {},
            }

            i += 1;
        }
    }
    
}