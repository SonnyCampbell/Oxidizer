use std::collections::HashMap;

use strum::EnumCount;

use crate::oscillator::Oscillator;
use crate::time;
use crate::envelope::EnvelopeADSR;
use crate::note_generator::{NoteGenerator, NoteOscillatorParams};
use crate::constants::*;

pub struct SoundGenerator {
    held_notes: HashMap<i32, NoteGenerator>,
    released_notes: Vec<NoteGenerator>,
    generators: [SoundGenOscParams; OscNumber::COUNT]
}


impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        return SoundGenerator {
            held_notes: HashMap::new(),
            released_notes: Vec::new(),
            generators: SoundGenOscParams::create_default_array(),
        }
    }

    pub fn note_released(&mut self, note: i32){
        if let Some(mut removed) = self.held_notes.remove(&note) {
            removed.note_released();
            self.released_notes.push(removed);
        }
    }



    fn get_note_params(&self) -> [Option<NoteOscillatorParams>; OscNumber::COUNT] {
        let mut osc_params: Vec<Option<NoteOscillatorParams>> = Vec::with_capacity(OscNumber::COUNT);

        for osc in &self.generators {
            if osc.enabled {
                osc_params.push(Some(NoteOscillatorParams::new(osc.wave_type, osc.unisons, osc.unison_detune_pct)));
            } else {
                osc_params.push(None);
            }
        }

        return osc_params.try_into()
            .unwrap_or_else(|v: Vec<Option<NoteOscillatorParams>>| 
                panic!("Expected a Vec of length {} but it was {}", OscNumber::COUNT, v.len()));

        
    }
    
    pub fn note_pressed(&mut self, note: i32){
        let note_gen: NoteGenerator = NoteGenerator::new(note, self.get_note_params());
        self.held_notes.insert(note, note_gen);
    }

    pub fn update_oscillator_params(&mut self, osc_params: SoundGenOscParams){
        let osc = &mut self.generators[osc_params.num as usize];

        osc.enabled = osc_params.enabled;
        osc.wave_type = osc_params.wave_type;
        osc.unisons = osc_params.unisons;
        osc.unison_detune_pct = osc_params.unison_detune_pct;

        self.update_note_params();
    }

    // todo: refactor: we only need to do this for the oscillator that had its params updated
    fn update_note_params(&mut self){
        let note_params = self.get_note_params();

        for note_gen in &mut self.held_notes {
            note_gen.1.set_note_params(&note_params)
        }

        for note_gen in &mut self.released_notes {
            note_gen.set_note_params(&note_params)
        }
    }

    pub fn get_sample(&mut self, envelope: &EnvelopeADSR, lfo: Option<&Oscillator>) -> f32 {
        let mut total = 0.0;
        let time = time::get_time();

        let mut lfo_freq = 0.0;
        let mut lfo_amplitude = 0.0;

        if let Some(lfo_osc) = lfo {
            lfo_freq = lfo_osc.get_frequency();
            lfo_amplitude = lfo_osc.get_amplitude();
        } 

        for note_gen in &mut self.held_notes {
            let amplitude = envelope.get_amplitude(time, note_gen.1.trigger_on_time, note_gen.1.trigger_off_time, note_gen.1.note_pressed);
            total += note_gen.1.get_sample(lfo_freq, lfo_amplitude) * amplitude;
        }

        let mut i = 0;
        let mut finished: Vec<usize> = Vec::new();
        
        for note_gen in &mut self.released_notes{
            
            let amplitude = envelope.get_amplitude(time, note_gen.trigger_on_time, note_gen.trigger_off_time, note_gen.note_pressed);
            if amplitude > 0.0 {
                total += note_gen.get_sample(lfo_freq, lfo_amplitude) * amplitude;
            }
            else {
                finished.push(i);
            }
            
            i += 1;
        }

        finished.reverse();
        for remove_index in finished {
            self.released_notes.remove(remove_index);
        }

        return total;
    }
}