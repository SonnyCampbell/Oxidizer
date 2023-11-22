use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use std::time::Duration;

fn main() {
    // Set the sample rate and frequency of the sawtooth wave
    let sample_rate = 44100;
    let frequency = 440.0;

    // Calculate the duration of the audio in seconds
    let duration = Duration::from_secs_f32(2.0);

    // Generate a sawtooth wave buffer
    // let buffer: Vec<_> = {
    //     let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    //     (0..num_samples)
    //         .map(|i| (i as f32 * frequency * 2.0 * std::f32::consts::PI / sample_rate as f32).sin())
    //         .collect()
    // };

    // Generate a triangle wave buffer
    // let buffer: Vec<_> = {
    //     let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    //     let mut samples = Vec::with_capacity(num_samples);

    //     for i in 0..num_samples {
    //         let t = (i as f32 / sample_rate as f32) * frequency;
    //         let normalized_t = t - normalized_floor(t);
    //         samples.push(1.0 - 4.0 * normalized_t.abs());
    //     }

    //     samples
    // };

    // Generate a sine wave buffer
    // let buffer: Vec<_> = {
    //     let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    //     let samples = (0..num_samples)
    //         .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * frequency / sample_rate as f32).sin())
    //         .collect::<Vec<_>>();
    //     samples
    // };

    // Generate a square wave buffer
    // let buffer: Vec<_> = {
    //     let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    //     let samples = (0..num_samples)
    //         .map(|i| {
    //             if (i as f32 * frequency / sample_rate as f32).sin() > 0.0 {
    //                 1.0
    //             } else {
    //                 -1.0
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //     samples
    // };

    // Set the duty cycle (0.0 to 1.0, where 0.5 is a square wave)
    let duty_cycle = 0.5;

    // Generate a pulse wave buffer
    let buffer: Vec<_> = {
        let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
        let samples = (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                if (t * frequency) % 1.0 < duty_cycle {
                    1.0
                } else {
                    -1.0
                }
            })
            .collect::<Vec<_>>();
        samples
    };

    // Set up the audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    // Convert the buffer to a SamplesBuffer and append it to the sink
    let samples_buffer = SamplesBuffer::new(1, sample_rate, buffer);
    sink.append(samples_buffer);

    // Sleep to allow the audio to play
    std::thread::sleep(duration);

    // Manually stop the sink to release resources
    sink.stop();
}

// Helper function to calculate the floor of a normalized value
fn normalized_floor(x: f32) -> f32 {
    x.floor()
}
