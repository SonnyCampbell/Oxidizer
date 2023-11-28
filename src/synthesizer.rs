use std::sync::mpsc::Receiver;
use std::time::Duration;


use rodio::Source;

use crate::constants::*;
use crate::envelope::EnvelopeADSR;
use crate::wavetype::WaveType;
use crate::sound_generator::SoundGenerator;



pub enum EnvelopeParam {
    AttackTime,
    DecayTime,
    ReleaseTime
}

pub enum SynthEvent {
    NotePress (i32),
    NoteRelease (i32),
    ChangeWaveType (OscNumber, WaveType),
    ChangeEnvelope (EnvelopeParam, f32)
}


pub struct Synthesizer {
    receiver: Receiver<SynthEvent>,
    sound_generator: SoundGenerator,


    envelope: EnvelopeADSR, 
    lfo_freq: f32,
    lfo_amplitude: f32
}

impl Synthesizer {
    pub fn new(receiver: Receiver<SynthEvent>) -> Synthesizer {
        return Synthesizer{
            receiver: receiver,
            sound_generator: SoundGenerator::new(),

            envelope: EnvelopeADSR::new(),
            lfo_freq: 0.0,
            lfo_amplitude: 0.0
        };
    }

    fn set_attack_time(&mut self, attack: f32){
        self.envelope.set_attack_time(attack);
    }

    fn set_decay_time(&mut self, decay: f32){
        self.envelope.set_decay_time(decay);
    }

    fn set_release_time(&mut self, release: f32){
        self.envelope.set_release_time(release);
    }

    fn handle_events(&mut self) {
        if let Ok(event) = self.receiver.try_recv(){
            match event {
                SynthEvent::NotePress(note) => self.sound_generator.note_pressed(note),
                SynthEvent::NoteRelease(note) => self.sound_generator.note_released(note),
                SynthEvent::ChangeWaveType(osc_num, wave_type) => self.sound_generator.changed_wave_type(osc_num, wave_type),
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


    pub fn get_synth_sample(&mut self) -> f32 {
        self.handle_events();

        return self.sound_generator.get_sample(&self.envelope, self.lfo_freq, self.lfo_amplitude);
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