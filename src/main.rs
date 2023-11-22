use rodio::{OutputStream, Sink};
use console::Term;

use std::time::Duration;
use std::thread;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

mod wavetable;
use wavetable::WaveTable;

mod wavetype;
use wavetype::WaveType;



fn main() {
    println!("Press 1 for Sin wave, 2 for Saw wave, etc...");

    let stdout = Term::buffered_stdout();

    let mut is_playing = false;
    let mut stop_playing = false;
    let mut start_playing = false;
    
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    'program_loop: loop {
        let mut wave_table = WaveTable::new(64, WaveType::Sine);
        

        if let Ok(input) = stdout.read_char() {
            match input {
                '1' => wave_table.set_wave_type(WaveType::Sine),
                '2' => wave_table.set_wave_type(WaveType::Saw),
                '3' => wave_table.set_wave_type(WaveType::Tri),
                '4' => wave_table.set_wave_type(WaveType::Square),
                '5' => wave_table.set_wave_type(WaveType::Pulse),
                'a' => stop_playing = true,
                '0' => { break 'program_loop },
                _ => wave_table.set_wave_type(WaveType::Sine),
            }
        }
        
        let mut oscillator = WavetableOscillator::new(44100, wave_table);
        oscillator.set_frequency(220.0);
        oscillator.set_gain(-30.0);

        
        if is_playing && stop_playing {
            sink.stop();
            start_playing = false;
        }
        
        if !is_playing && start_playing {
            sink.append(oscillator);
            sink.play();
            is_playing = true;
            start_playing = false;
        }

        
        
    }
    
}

