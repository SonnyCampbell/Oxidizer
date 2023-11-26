pub struct EnvelopeADSR{
    attack_time: f32,
    decay_time: f32,
    release_time: f32,
    sustain_amplitude: f32,
    start_amplitude: f32,
}

impl EnvelopeADSR{
    pub fn new() -> EnvelopeADSR {
        return EnvelopeADSR{
            attack_time: 1.0,
            decay_time: 1.0,
            release_time: 2.0,
            sustain_amplitude: 0.1,
            start_amplitude: 0.11,
        }
    }

    pub fn get_amplitude(&self, time: f32, trigger_on_time: f32, trigger_off_time: f32, note_pressed: bool) -> f32 {
        let mut amp = 0.0;

        if note_pressed {
            let lifetime = time - trigger_on_time;

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
            let mut release_amplitude = 0.0;
            let lifetime = trigger_off_time - trigger_on_time;
            // Never reached full amplitude
            if lifetime <= self.attack_time {
                release_amplitude = (lifetime / self.attack_time) * self.start_amplitude; 
            }
            else if lifetime > self.attack_time && lifetime <= self.decay_time {
                release_amplitude = ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude;
            }
            else { // lifetime > self.attack_time + self.decay_time
                release_amplitude = self.sustain_amplitude;
            }

            amp = ((time - trigger_off_time) / self.release_time) * (0.0 - release_amplitude) + release_amplitude;

            
        }
        
        if amp <= 0.0001 {
            amp = 0.0;
        }


        return amp;
    }

    pub fn set_attack_time(&mut self, attack: f32){
        self.attack_time = attack;
    }

    pub fn set_decay_time(&mut self, decay: f32){
        self.decay_time = decay;
    }

    pub fn set_release_time(&mut self, release: f32){
        self.release_time = release;
    }
}