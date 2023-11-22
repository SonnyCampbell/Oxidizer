use rodio::{OutputStream, Sink};
use std::f32::consts::PI;
use std::time::Duration;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;


fn main() {
    let wave_table_size: usize = 64; //why is this 64?
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    // fill wavetable with values of sine wave.
    // change this to fill wave table with values of selected wave type
    for n in 0..wave_table_size {
        let t = n as f32 / wave_table_size as f32;

        // sine wave
        wave_table.push((2.0 * PI * t).sin());
        
        // saw wave
        //wave_table.push(((t + PI) / PI) % 2.0 - 1.0);

        // triangle wave
        //wave_table.push(1.0 - (t - 0.5).abs()*4.0)

        // square wave
        //wave_table.push((2.0 * PI * t).sin().signum());

        // pulse wave
        // let duty_cycle = 0.2;
        // if t % 1.0 < duty_cycle {
        //     wave_table.push(1.0)
        // } else {
        //     wave_table.push(-1.0)
        // }
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(220.0);
    oscillator.set_gain(-20.0);

    // Set up the audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(oscillator);
    //sink.sleep_until_end();
    sink.play();
    //let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(Duration::from_secs(3));
    
}