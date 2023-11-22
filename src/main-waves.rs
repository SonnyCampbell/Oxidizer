use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    let sample_rate = 44100;
    let frequency = 220.0;

    let _sin_wave_samples: Vec<f32> = (0..sample_rate)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (t * frequency * 2.0 * PI).sin()
        })
        .collect();

    let _saw_wave_samples: Vec<f32> = (0..sample_rate)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            let inner = (t) % (1.0 / frequency);
            let result = (2.0 * inner * frequency) - 1.0;
            return result;
        })
        .collect();

    let _triangle_wave_samples: Vec<f32> = (0..sample_rate)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;

            let inner = (t * frequency - (t * frequency + 0.5).floor()).abs();

            let result = (4.0 * inner) - 1.0;
            return result;
        })
        .collect();

    let _square_wave_samples: Vec<f32> = (0..sample_rate)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (t * frequency * 2.0 * PI).sin().signum()
        })
        .collect();


    let duty_cycle = 0.2;
    let _pulse_wave_samples: Vec<f32> = (0..sample_rate)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            if (t * frequency) % 1.0 < duty_cycle {
                1.0
            } else {
                -1.0
            }
        }).collect();

    // Set up the audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    // Convert the FM wave to a SamplesBuffer and append it to the sink
    let samples_buffer = SamplesBuffer::new(1, sample_rate as u32, _pulse_wave_samples);
    sink.append(samples_buffer);
    sink.sleep_until_end();

    // Manually stop the sink to release resources
    sink.stop();
}

fn _do_thing(){
    // Set the sample rate and frequencies of the carrier and modulator waves
    let sample_rate = 44100;
    let carrier_frequency = 440.0;
    let modulator_frequency = 220.0; // Modulator frequency in Hertz
    let modulation_index = 1.0; // Modulation index

    // Calculate the duration of the audio in seconds
    let duration = Duration::from_secs_f32(2.0);

    // Generate the carrier and modulator waves
    let carrier_wave: Vec<_> = generate_sine_wave(carrier_frequency, sample_rate, duration);
    let modulator_wave: Vec<_> = generate_sine_wave(modulator_frequency, sample_rate, duration);

    // Perform frequency modulation
    let fm_wave: Vec<_> = carrier_wave
        .iter()
        .zip(modulator_wave.iter())
        .map(|(&carrier_sample, &modulator_sample)| {
            let modulation = modulation_index * modulator_sample;
            carrier_sample * (2.0 * PI * modulation).sin()
        })
        .collect();

    // Set up the audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    // Convert the FM wave to a SamplesBuffer and append it to the sink
    let samples_buffer = SamplesBuffer::new(1, sample_rate as u32, fm_wave);
    sink.append(samples_buffer);

    // Sleep to allow the audio to play
    std::thread::sleep(duration);

    // Manually stop the sink to release resources
    sink.stop();
}

// Helper function to generate a sine wave
fn generate_sine_wave(frequency: f32, sample_rate: usize, duration: Duration) -> Vec<f32> {
    let num_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
    (0..num_samples)
        .map(|i| (i as f32 * 2.0 * PI * frequency / sample_rate as f32).sin())
        .collect()
}
