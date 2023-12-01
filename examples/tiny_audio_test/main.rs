use std::f32::consts::PI;

use tinyaudio::prelude::*;

fn main() {

    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    let mut test = TestNoise::new();

    let _device = run_output_device(params, {
        //let mut clock = 0f32;
        move |data| {
            for samples in data.chunks_mut(params.channels_count) {
                // clock = (clock + 1.0) % params.sample_rate as f32;
                // let value =
                //     (clock * 440.0 * 2.0 * std::f32::consts::PI / params.sample_rate as f32).sin();

                let value = test.next().unwrap();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    })
    .unwrap();
    
    std::thread::sleep(std::time::Duration::from_secs(5));
}

struct TestNoise {
    sample_index: f32,
    frequency: f32,
}

impl TestNoise {
    fn new() -> TestNoise{
        return TestNoise { sample_index: 0.0, frequency: 440.0 }
    }
}

impl Iterator for TestNoise {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let w = self.frequency * 2.0 * PI * (self.sample_index / 44100.0);
        self.sample_index += 1.0;
        self.sample_index %= 44100.0;

        let volume = 0.03;
        return Some(w.sin() * volume);
    }
}