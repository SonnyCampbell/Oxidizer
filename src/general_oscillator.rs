use crate::time;
use crate::envelope::EnvelopeADSR;
use crate::oscillator::Oscillator;
use crate::wavetype::WaveType;

pub struct GeneralOscillator {
    note_oscillator: Oscillator,
    envelope: EnvelopeADSR, //Todo: envelope shouldn't live on the oscillator, 
    //it should live on the synth and be applied to all notes played by the same oscillator
}

impl GeneralOscillator {
    pub fn new(freq: f32, sample_rate: f32, wave_type: WaveType) -> GeneralOscillator {
        let mut oscillator = GeneralOscillator{
            note_oscillator: Oscillator::new(freq, sample_rate, wave_type),
            envelope: EnvelopeADSR::new()
        };

        oscillator.note_pressed();
        return oscillator;
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType){
        self.note_oscillator.set_wave_type(wave_type);
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