use rodio::{OutputStream, Sink};
use console::Term;

use std::time::Duration;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

mod wavetable;
use wavetable::WaveTable;

mod wavetype;
use wavetype::WaveType;



fn main() {
    println!("Press 1 for Sin wave, 2 for Saw wave, etc...");

    let stdout = Term::buffered_stdout();

    let mut wave_table = WaveTable::new(64, WaveType::Sine);
    
    'program_loop: loop {
        

        if let Ok(input) = stdout.read_char() {
            match input {
                '1' => wave_table.set_wave_type(WaveType::Sine),
                '2' => wave_table.set_wave_type(WaveType::Saw),
                '3' => wave_table.set_wave_type(WaveType::Tri),
                '4' => wave_table.set_wave_type(WaveType::Square),
                '5' => wave_table.set_wave_type(WaveType::Pulse),
                '0' => { break 'program_loop },
                _ => wave_table.set_wave_type(WaveType::Sine),
            }
        }
        
        let mut oscillator = WavetableOscillator::new(44100, wave_table);
        oscillator.set_frequency(220.0);
        oscillator.set_gain(-30.0);

        // Set up the audio output stream
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
        
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(oscillator);
        //sink.sleep_until_end();
        sink.play();
        //let _result = stream_handle.play_raw(oscillator.convert_samples());
        std::thread::sleep(Duration::from_secs(3));
    }
    
}

