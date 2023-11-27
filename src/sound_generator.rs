use std::collections::HashMap;

use crate::time;
use crate::envelope::EnvelopeADSR;
use crate::wavetype::WaveType;
use crate::note_generator::NoteGenerator;
use crate::constants::*;

pub struct SoundGenerator {
    held_notes: HashMap<i32, NoteGenerator>,
    released_notes: Vec<NoteGenerator>,
    wave_type: WaveType,
}


impl SoundGenerator {
    pub fn new() -> SoundGenerator {
        return SoundGenerator {
            held_notes: HashMap::new(),
            released_notes: Vec::new(),
            wave_type: WaveType::default(),
        }
    }

    pub fn note_released(&mut self, note: i32){
        if let Some(mut removed) = self.held_notes.remove(&note) {
            removed.note_released();
            self.released_notes.push(removed);
        }
    }

    fn get_frequency(i: f32) -> f32{
        let base_frequency = 220.0;
        let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
        return base_frequency * twelfth_root_of_two.powf(i as f32);
    }

    
    pub fn note_pressed(&mut self, note: i32){
        let freq = Self::get_frequency(note as f32);
        let note_gen: NoteGenerator = NoteGenerator::new(freq, SAMPLE_RATE, self.wave_type.clone());
        self.held_notes.insert(note, note_gen);
    }

    pub fn changed_wave_type(&mut self, wave_type: WaveType){
        self.wave_type = wave_type;

        for osc in &mut self.held_notes {
            osc.1.set_wave_type(self.wave_type.clone())
        }

        for osc in &mut self.released_notes {
            osc.set_wave_type(self.wave_type.clone())
        }
    }

    pub fn get_sample(&mut self, envelope: &EnvelopeADSR, lfo_freq: f32, lfo_amplitude: f32) -> f32 {
        let mut total = 0.0;
        let time = time::get_time();

        for osc in &mut self.held_notes {
            let amplitude = envelope.get_amplitude(time, osc.1.trigger_on_time, osc.1.trigger_off_time, osc.1.note_pressed);
            total += osc.1.get_sample(lfo_freq, lfo_amplitude) * amplitude;
        }

        let mut i = 0;
        let mut finished: Vec<usize> = Vec::new();
        
        for osc in &mut self.released_notes{
            
            let amplitude = envelope.get_amplitude(time, osc.trigger_on_time, osc.trigger_off_time, osc.note_pressed);
            if amplitude > 0.0 {
                total += osc.get_sample(lfo_freq, lfo_amplitude) * amplitude;
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