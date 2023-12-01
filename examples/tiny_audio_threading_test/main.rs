use std::f32::consts::PI;

use tinyaudio::prelude::*;

fn main() {

    let params = OutputDeviceParameters {
        channels_count: 1,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    let mut test = TestNoise::new();

    
    
    let _ = std::thread::spawn(move || {
        let _device = run_output_device(params, {
            move |data| {
                for samples in data.chunks_mut(params.channels_count) {
                    let value = test.next().unwrap();
                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        })
        .unwrap();

        loop {}
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Audio running on other thread.");
    }

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