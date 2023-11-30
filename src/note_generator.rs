use strum::EnumCount;

use crate::constants::OscNumber;
use crate::time;
use crate::oscillator::Oscillator;
use crate::wavetype::WaveType;


const UNISON_MAX_NOTE_DETUNE: f32 = 2.0;

pub struct NoteOscillatorParams {
    wave_type: WaveType,
    unisons: i32,
    unison_detune_pct: f32
}

impl NoteOscillatorParams {
    pub fn new(wave_type: WaveType, unisons: i32, unison_detune_pct: f32) -> NoteOscillatorParams {
        return NoteOscillatorParams{
            wave_type, unisons, unison_detune_pct
        };
    }
}

pub struct NoteGenerator {
    pub trigger_on_time: f32,
    pub trigger_off_time: f32,
    pub note_pressed: bool,
    
    oscillators: [Option<Vec<Oscillator>>; OscNumber::COUNT]
}

impl NoteGenerator {
    pub fn new(note: i32, note_params: [Option<NoteOscillatorParams>; OscNumber::COUNT]) -> NoteGenerator {
        const INIT: Option<Vec<Oscillator>> = None;
        let mut oscillators: [Option<Vec<Oscillator>>; OscNumber::COUNT] = [INIT; OscNumber::COUNT];

        
        let mut i = 0;
        for opt in note_params { //todo: use enumerate to get index
            match opt {
                Some(param) => {
                    let mut osc_vec: Vec<Oscillator> = Vec::new();

                    if param.unisons % 2 == 0 {
                        let unisons_to_add = param.unisons / 2;
                        Self::add_unison_oscillators(&mut osc_vec, note, unisons_to_add, param.unison_detune_pct, param.wave_type);

                    } else {
                        let freq = Self::get_frequency(note as f32);
                        osc_vec.push(Oscillator::new(freq, param.wave_type));

                        if param.unisons > 1 {
                            let unisons_to_add = (param.unisons - 1) / 2;
                            Self::add_unison_oscillators(&mut osc_vec, note, unisons_to_add, param.unison_detune_pct, param.wave_type);
                        }
                    }

                    oscillators[i] = Some(osc_vec);
                },
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

    
    fn add_unison_oscillators(osc_vec: &mut Vec<Oscillator>, note: i32, unisons_to_add: i32, detune_pct: f32, wave_type: WaveType){
        for i in 0..unisons_to_add {
            let note_detune = detune_pct * UNISON_MAX_NOTE_DETUNE / (2.0 as f32).powi(i);

            let above = Self::get_frequency(note as f32 + note_detune);
            osc_vec.push(Oscillator::new(above, wave_type));

            let below = Self::get_frequency(note as f32 - note_detune);
            osc_vec.push(Oscillator::new(below, wave_type));
        }
    }

    fn get_frequency(i: f32) -> f32{
        let base_frequency = 220.0;
        let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
        return base_frequency * twelfth_root_of_two.powf(i as f32);
    }

    pub fn note_released(&mut self){
        self.trigger_off_time = time::get_time();
        self.note_pressed = false;
    }

    pub fn get_sample(&mut self, lfo_freq: f32, lfo_amplitude: f32) -> f32 {
        let mut total = 0.0;
        
        for opt in &mut self.oscillators {
            match opt {
                Some(osc_vec) => {
                    for osc in osc_vec {
                        total += osc.get_sample(lfo_freq, lfo_amplitude)
                    }
                },
                None => {},
            } 
        }

        return total;
    }

    pub fn set_note_params(&mut self, note_params: &[Option<NoteOscillatorParams>; OscNumber::COUNT]){
        let mut i = 0;
        for opt in note_params {
            match opt {
                Some(param) => {
                    if let Some(osc_vec) = &mut self.oscillators[i] {
                        for osc in osc_vec {
                            osc.set_wave_type(param.wave_type);
                        }
                        // todo: update unisons
                    }
                    
                },
                None => {},
            }

            i += 1;
        }
    }
    
}