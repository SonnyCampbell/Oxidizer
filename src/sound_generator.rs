use std::collections::{HashMap, VecDeque};

use strum::EnumCount;

use crate::oscillator::Oscillator;
use crate::time;
use crate::envelope::EnvelopeADSR;
use crate::note_generator::{NoteGenerator, NoteOscillatorParams};
use crate::constants::*;

pub struct SoundGenerator {
    held_notes: HashMap<i32, NoteGenerator>,
    released_notes: VecDeque<NoteGenerator>,
    finished_playing: Vec<usize>,
    generators: [SoundGenOscParams; OscNumber::COUNT]
}


impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        return SoundGenerator {
            held_notes: HashMap::with_capacity(MAX_NOTES),
            released_notes: VecDeque::with_capacity(MAX_NOTES),
            finished_playing: Vec::with_capacity(MAX_NOTES),
            generators: SoundGenOscParams::create_default_array(),
        }
    }

    pub fn note_released(&mut self, note: i32){
        if let Some(mut removed) = self.held_notes.remove(&note) {
            removed.note_released();

            if self.released_notes.len() >= MAX_NOTES {
                self.released_notes.pop_front();
            }

            self.released_notes.push_back(removed);
            
        }
    }

    fn get_note_params(&self) -> [Option<NoteOscillatorParams>; OscNumber::COUNT] {
        const INIT: Option<NoteOscillatorParams> = None;
        let mut osc_params: [Option<NoteOscillatorParams>; OscNumber::COUNT] = [INIT; OscNumber::COUNT];

        for (i, osc) in self.generators.iter().enumerate() {
            if osc.enabled {
                osc_params[i] = Some(NoteOscillatorParams::new(osc.wave_type, osc.unisons, osc.unison_detune_pct));
            } else {
                osc_params[i] = None;
            }
        }

        return osc_params;  
    }

    fn get_note_params_for_osc(&self, osc_num: usize) -> Option<NoteOscillatorParams> {
        let osc = &self.generators[osc_num];

        if !osc.enabled {
            return None;
        }

        return Some(NoteOscillatorParams::new(osc.wave_type, osc.unisons, osc.unison_detune_pct));
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

        self.update_note_params(osc_params.num as usize);
    }

    fn update_note_params(&mut self, osc_num: usize){
        let note_params = self.get_note_params_for_osc(osc_num);

        match note_params {
            Some(param) => {
                for note_gen in &mut self.held_notes {
                    note_gen.1.set_note_params(osc_num, &param);
                }
        
                for note_gen in &mut self.released_notes {
                    note_gen.set_note_params(osc_num, &param);
                }
            },
            None => {},
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

        for (i, note_gen) in  self.released_notes.iter_mut().enumerate() {
            
            let amplitude = envelope.get_amplitude(time, note_gen.trigger_on_time, note_gen.trigger_off_time, note_gen.note_pressed);
            if amplitude > 0.0 {
                total += note_gen.get_sample(lfo_freq, lfo_amplitude) * amplitude;
            }
            else {
                if self.finished_playing.len() <= MAX_NOTES {
                    self.finished_playing.push(i);
                }
            }
        }

        self.finished_playing.reverse();
        for remove_index in &self.finished_playing {
            self.released_notes.remove(*remove_index);
        }

        self.finished_playing.clear();

        return total;
    }
}