#[derive(Clone)]
pub struct EnvelopeADSR{
    attack_time: f32,
    decay_time: f32,
    release_time: f32,
    sustain_amplitude: f32,
    start_amplitude: f32,
    trigger_on_time: f32,
    trigger_off_time: f32,

    note_pressed: bool
}

impl EnvelopeADSR{
    pub fn new() -> EnvelopeADSR {
        return EnvelopeADSR{
            attack_time: 1.0,
            decay_time: 1.0,
            release_time: 2.0,
            sustain_amplitude: 0.8,
            start_amplitude: 1.0,
            trigger_on_time: 0.0,
            trigger_off_time: 0.0,
            note_pressed: false,
        }
    }

    pub fn get_amplitude(&self, time: f32) -> f32 {
        let mut amp = 0.0;

        let lifetime = time - self.trigger_on_time;

        if self.note_pressed {
            // ADS
            if lifetime <= self.attack_time {
                // Attack
                amp = (lifetime / self.attack_time) * self.start_amplitude; 
            }
            else if lifetime > self.attack_time && lifetime <= self.decay_time {
                // Decay
                amp = ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude;
            }
            else { // lifetime > self.attack_time + self.decay_time
                // Sustain
                amp = self.sustain_amplitude;
            }
        } 
        else {
            // Release
            amp = ((time - self.trigger_off_time) / self.release_time) * (0.0 - self.sustain_amplitude) + self.sustain_amplitude;
        }
        
        if amp <= 0.0001 {
            amp = 0.0;
        }


        return amp;
    }

    pub fn note_on(&mut self, time_on: f32){
        self.trigger_on_time = time_on;
        self.note_pressed = true;
    }

    pub fn note_off(&mut self, time_off: f32){
        self.trigger_off_time = time_off;
        self.note_pressed = false;
    }
}