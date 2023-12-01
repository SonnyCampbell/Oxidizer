
use rodio::{OutputStream, Source, Sink};
use std::f32::consts::PI;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let test = TestNoise::new();

    sink.append(test);
    sink.play();
    sink.sleep_until_end();

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

        let volume = 0.05;
        return Some(w.sin() * volume);
    }
}

impl Source for TestNoise {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}