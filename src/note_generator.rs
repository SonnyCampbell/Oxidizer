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
    note: i32,
    
    oscillators: [Option<Vec<Oscillator>>; OscNumber::COUNT]
}

impl NoteGenerator {
    pub fn new(note: i32, note_params: [Option<NoteOscillatorParams>; OscNumber::COUNT]) -> NoteGenerator {
        const INIT: Option<Vec<Oscillator>> = None;
        let mut oscillators: [Option<Vec<Oscillator>>; OscNumber::COUNT] = [INIT; OscNumber::COUNT];

        for (osc_num, opt) in note_params.iter().enumerate() { 
            match opt {
                Some(param) => {
                    let osc_unison_voices = Self::get_unison_voices_for_note(note, param, 0.0);
                    oscillators[osc_num] = Some(osc_unison_voices);
                },
                None => oscillators[osc_num] = None,
            }
        }

        return NoteGenerator{
            trigger_on_time: time::get_time(),
            trigger_off_time: 0.0,
            note_pressed: true,
            note,
            oscillators: oscillators,
        };
    }

    fn get_unison_voices_for_note(note: i32, param: &NoteOscillatorParams, starting_sample_index: f32) -> Vec<Oscillator> {
        let mut osc_unison_voices: Vec<Oscillator> = Vec::new();

        if param.unisons % 2 == 0 {
            let unisons_to_add = param.unisons / 2;
            Self::add_unison_oscillators_to_vec(&mut osc_unison_voices, note, unisons_to_add, param.unison_detune_pct, param.wave_type, starting_sample_index);

        } else {
            if param.unisons > 1 {
                let unisons_to_add = (param.unisons - 1) / 2;
                Self::add_unison_oscillators_to_vec(&mut osc_unison_voices, note, unisons_to_add, param.unison_detune_pct, param.wave_type, starting_sample_index);
            }

            let freq = Self::get_frequency(note as f32);
            osc_unison_voices.push(Oscillator::new(freq, param.wave_type, starting_sample_index));
        }

        return osc_unison_voices;
    }

    // fn get_note_detune -> f32 

    
    fn add_unison_oscillators_to_vec(osc_vec: &mut Vec<Oscillator>, note: i32, unisons_to_add: i32, detune_pct: f32, wave_type: WaveType, starting_sample_index: f32){
        for i in 0..unisons_to_add {
            let note_detune = detune_pct * UNISON_MAX_NOTE_DETUNE / (2.0 as f32).powi(i);

            let above = Self::get_frequency(note as f32 + note_detune);
            osc_vec.push(Oscillator::new(above, wave_type, starting_sample_index));

            let below = Self::get_frequency(note as f32 - note_detune);
            osc_vec.push(Oscillator::new(below, wave_type, starting_sample_index));
        }
    }

    fn set_unison_params(osc_unison_voices: &mut Vec<Oscillator>, note: i32, num_unisons: usize, note_params: &NoteOscillatorParams) {
        for i in 0..num_unisons {
            let note_detune = note_params.unison_detune_pct * UNISON_MAX_NOTE_DETUNE / (2.0 as f32).powi(i as i32);

            let above = Self::get_frequency(note as f32 + note_detune);
            osc_unison_voices[i * 2].set_frequency(above);
            osc_unison_voices[i * 2].set_wave_type(note_params.wave_type);

            let below = Self::get_frequency(note as f32 - note_detune);
            osc_unison_voices[(i * 2) + 1].set_frequency(below);
            osc_unison_voices[(i * 2) + 1].set_wave_type(note_params.wave_type);
        }
    }

    pub fn set_note_params(&mut self, osc_num: usize, note_params: &NoteOscillatorParams){

        let mut starting_sample_index = 0.0;
        let mut current_unison_voice_count = 0;
        if let Some(osc_unison_voices) = &mut self.oscillators[osc_num] {
            if let Some(voice) = osc_unison_voices.first() {
                starting_sample_index = voice.get_sample_index();
            }

            current_unison_voice_count = osc_unison_voices.len();
        }

        //todo: potentially refactor
        //unisons start from the outside in 2/1/0.5/0.25/etc...
        //if we removed unisons only need to remove the inner voices
        //if we added unisons only need to add the inner voices
        if note_params.unisons as usize != current_unison_voice_count {
            let unison_voices = Self::get_unison_voices_for_note(self.note, note_params, starting_sample_index);
            self.oscillators[osc_num] = Some(unison_voices);
        }
        else {
            if let Some(osc_unison_voices) = &mut self.oscillators[osc_num] {
                if current_unison_voice_count % 2 == 0 {
                    let num_unisons = current_unison_voice_count / 2;
                    Self::set_unison_params(osc_unison_voices, self.note, num_unisons, note_params);

                } else {
                    let num_unisons = (current_unison_voice_count - 1) / 2;
                    Self::set_unison_params(osc_unison_voices, self.note, num_unisons, note_params);
                }
            }
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
    
}