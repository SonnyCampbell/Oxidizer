use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::collections::HashMap;

use rodio::Source;

use crate::time;
use crate::envelope::EnvelopeADSR;
use crate::wavetype::WaveType;
use crate::general_oscillator::GeneralOscillator;

static SAMPLE_RATE: f32 = 44100.0;
static NUM_CHANNELS: u16 = 1;



pub enum EnvelopeParam {
    AttackTime,
    DecayTime,
    ReleaseTime
}

pub enum SynthEvent {
    NotePress (i32),
    NoteRelease (i32),
    ChangeWaveType (WaveType),
    ChangeEnvelope (EnvelopeParam, f32)
}


pub struct Synthesizer {
    receiver: Receiver<SynthEvent>,
    held_oscillators: HashMap<i32, GeneralOscillator>,
    released_oscillators: Vec<GeneralOscillator>,
    wave_type: WaveType,
    attack: f32,
    decay: f32,
    release: f32,

    envelope: EnvelopeADSR, 
}

impl Synthesizer {
    pub fn new(receiver: Receiver<SynthEvent>) -> Synthesizer {
        return Synthesizer{
            receiver: receiver,
            held_oscillators: HashMap::new(),
            released_oscillators: Vec::new(),
            wave_type: WaveType::default(),
            attack: 1.0,
            decay: 1.0,
            release: 2.0,
            envelope: EnvelopeADSR::new()
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
        let osc = GeneralOscillator::new(freq, SAMPLE_RATE, self.wave_type.clone());
        self.held_oscillators.insert(note, osc);
    }

    fn set_attack_time(&mut self, attack: f32){
        self.attack = attack;
        self.envelope.set_attack_time(attack); // attack/decay/release no longer live on the synth class?
    }

    fn set_decay_time(&mut self, decay: f32){
        self.decay = decay;
        self.envelope.set_decay_time(decay);
    }

    fn set_release_time(&mut self, release: f32){
        self.release = release;
        self.envelope.set_release_time(release);
    }

    fn changed_wave_type(&mut self, wave_type: WaveType){
        self.wave_type = wave_type;

        for osc in &mut self.held_oscillators {
            osc.1.set_wave_type(self.wave_type.clone())
        }

        for osc in &mut self.released_oscillators {
            osc.set_wave_type(self.wave_type.clone())
        }
    }

    fn handle_events(&mut self) {
        if let Ok(event) = self.receiver.try_recv(){
            match event {
                SynthEvent::NotePress(note) => self.note_pressed(note),
                SynthEvent::NoteRelease(note) => self.note_released(note),
                SynthEvent::ChangeWaveType(wave_type) => self.changed_wave_type(wave_type),
                SynthEvent::ChangeEnvelope(param, value) => {
                    match param {
                        EnvelopeParam::AttackTime => self.set_attack_time(value),
                        EnvelopeParam::DecayTime => self.set_decay_time(value),
                        EnvelopeParam::ReleaseTime => self.set_release_time(value),
                    }
                },
            }
        }
    }

    fn get_combined_sample(&mut self) -> f32 {
        let mut total = 0.0;
        let time = time::get_time();

        for osc in &mut self.held_oscillators {
            let amplitude = self.envelope.get_amplitude(time, osc.1.trigger_on_time, osc.1.trigger_off_time, osc.1.note_pressed);
            total += osc.1.get_sample() * amplitude;
        }

        let mut i = 0;
        let mut finished: Vec<usize> = Vec::new();
        
        for osc in &mut self.released_oscillators{
            
            let amplitude = self.envelope.get_amplitude(time, osc.trigger_on_time, osc.trigger_off_time, osc.note_pressed);
            if amplitude > 0.0 {
                total += osc.get_sample() * amplitude;
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
        return SAMPLE_RATE as u32;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}