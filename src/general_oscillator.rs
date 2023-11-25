use rodio::Source;
use std::time::Duration;

use crate::time;
use crate::wavetable::WaveTable;
use crate::envelope::EnvelopeADSR;
use crate::wavetable_oscillator::WavetableOscillator;

pub struct GeneralOscillator {
    note_oscillator: WavetableOscillator,
    envelope: EnvelopeADSR, //Todo: envelope shouldn't live on the oscillator, 
    //it should live on the synth and be applied to all notes played by the same oscillator
}

impl GeneralOscillator {
    pub fn new(freq: f32, sample_rate: u32, wavetable: &'static WaveTable) -> GeneralOscillator {
        let mut oscillator = GeneralOscillator{
            note_oscillator: WavetableOscillator::new(sample_rate, wavetable),
            envelope: EnvelopeADSR::new()
        };

        oscillator.note_pressed();
        oscillator.note_oscillator.set_frequency(freq);
        return oscillator;
    }

    pub fn set_wave_table(&mut self, wave_table: &'static WaveTable){
        self.note_oscillator.set_wave_table(wave_table);
    }

    fn note_pressed(&mut self){
        self.envelope.note_on(time::get_time());
    }

    pub fn note_released(&mut self){
        self.envelope.note_off(time::get_time());
    }

    pub fn get_amplitude(&self) -> f32 {
        return self.envelope.get_amplitude(time::get_time());
    }

    pub fn get_sample(&mut self) -> f32 {
        return self.get_amplitude() * self.note_oscillator.get_sample();
    }

    pub fn set_attack_time(&mut self, attack: f32){
        self.envelope.set_attack_time(attack);
    }

    pub fn set_decay_time(&mut self, decay: f32){
        self.envelope.set_decay_time(decay);
    }

    pub fn set_release_time(&mut self, release: f32){
        self.envelope.set_release_time(release);
    }
}

impl Iterator for GeneralOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32>{

        if self.get_amplitude() <= 0.0 {
            println!("Finisehd");
            return None;
        }

        //println!("Iterating {}", 1);

        return Some(self.get_sample());
    }
}

impl Source for GeneralOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return 44100;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}