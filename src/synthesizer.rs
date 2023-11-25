use std::sync::mpsc::*;
use std::time::Duration;
use std::collections::HashMap;

use rodio::Source;
use lazy_static::lazy_static;

use crate::wavetype::WaveType;
use crate::wavetable::WaveTables;
use crate::general_oscillator::GeneralOscillator;

static SAMPLE_RATE: u32 = 44100;
static NUM_CHANNELS: u16 = 1;

lazy_static! {
    static ref WAVE_TABLES: WaveTables = WaveTables::new();
}

pub enum SynthEvent {
    NotePress (i32),
    NoteRelease (i32),
    ChangeWaveType (WaveType)
}


pub struct Synthesizer {
    receiver: Receiver<SynthEvent>,
    held_oscillators: HashMap<i32, GeneralOscillator>,
    released_oscillators: Vec<GeneralOscillator>,
    wave_type: WaveType,

    wave_tables: &'static WaveTables,
}

impl Synthesizer {
    pub fn new(receiver: Receiver<SynthEvent>) -> Synthesizer {
        return Synthesizer{
            receiver: receiver,
            held_oscillators: HashMap::new(),
            released_oscillators: Vec::new(),
            wave_type: WaveType::default(),

            wave_tables: &WAVE_TABLES,
        };
    }

    fn get_frequency(i: f32) -> f32{
        let base_frequency = 220.0;
        let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
        return base_frequency * twelfth_root_of_two.powf(i as f32);
    }

    fn note_released(&mut self, note: i32){
        if let Some(mut removed) = self.held_oscillators.remove(&note) {
            removed.note_released();
            self.released_oscillators.push(removed);
        }
    }

    fn note_pressed(&mut self, note: i32){
        let freq = Self::get_frequency(note as f32);
        let wave_table = self.wave_tables.get_wave_table(&self.wave_type);
        let osc = GeneralOscillator::new(freq, 44100, wave_table);
        self.held_oscillators.insert(note, osc);
    }

    fn changed_wave_type(&mut self, wave_type: WaveType){
        self.wave_type = wave_type;
        let wave_table = self.wave_tables.get_wave_table(&self.wave_type);

        for osc in &mut self.held_oscillators {
            osc.1.set_wave_table(wave_table)
        }

        for osc in &mut self.released_oscillators {
            osc.set_wave_table(wave_table);
        }
    }

    fn handle_events(&mut self) {
        if let Ok(event) = self.receiver.try_recv(){
            match event {
                SynthEvent::NotePress(note) => self.note_pressed(note),
                SynthEvent::NoteRelease(note) => self.note_released(note),
                SynthEvent::ChangeWaveType(wave_type) => self.changed_wave_type(wave_type),
            }
        }
    }

    fn get_combined_sample(&mut self) -> f32 {
        let mut total = 0.0;

        for osc in &mut self.held_oscillators {
            total += osc.1.get_sample();
        }

        let mut i = 0;
        let mut finished: Vec<usize> = Vec::new();
        for osc in &mut self.released_oscillators{
            
            if osc.get_amplitude() > 0.0 {
                total += osc.get_sample();
            }
            else {
                finished.push(i);
            }
            
            i += 1;
        }

        finished.reverse();
        for remove_index in finished {
            self.released_oscillators.remove(remove_index);
        }

        return total;
    }

    pub fn get_synth_sample(&mut self) -> f32 {
        self.handle_events();

        return self.get_combined_sample();
    }
}

impl Iterator for Synthesizer {
    type Item = f32;

    fn next(&mut self) -> Option<f32>{
        return Some(self.get_synth_sample());
    }
}

impl Source for Synthesizer {
    fn channels(&self) -> u16 {
        return NUM_CHANNELS;
    }

    fn sample_rate(&self) -> u32 {
        return SAMPLE_RATE;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}