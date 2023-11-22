use rodio::source::{Source, SineWave};
use rodio::{OutputStream};
use std::time::Duration;

struct FrequencyModulation {
    carrier: SineWave,
    modulator: SineWave,
    modulation_index: f32,
    sample_rate: u32,
    phase: f32,
}

impl FrequencyModulation {
    fn new(carrier_freq: f32, modulator_freq: f32, modulation_index: f32, sample_rate: u32) -> Self {
        FrequencyModulation {
            carrier: SineWave::new(carrier_freq),
            modulator: SineWave::new(modulator_freq),
            modulation_index,
            sample_rate,
            phase: 0.0,
        }
    }
}

impl Source for FrequencyModulation {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.carrier.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for FrequencyModulation {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let modulator_sample = self.modulator.next()?;
        let modulated_freq = self.carrier.sample_rate() as f32
            + modulator_sample * self.modulation_index;

        self.carrier = SineWave::new(modulated_freq);
        self.phase = (self.phase + modulated_freq / self.sample_rate as f32) % 1.0;

        Some(self.carrier.next().unwrap_or(0.0))
    }
}

fn main() {
    let carrier_freq = 440.0;
    let modulator_freq = 220.0;
    let modulation_index = 2.0;

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let modulated_wave = FrequencyModulation::new(carrier_freq, modulator_freq, modulation_index, 44100);

    sink.append(modulated_wave);
    sink.sleep_until_end();

    println!("Hello, world!");
}


