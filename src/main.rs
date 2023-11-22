use rodio::{OutputStream, Sink};
use windows;
use ::phf::{Map, phf_map};

use std::time::Duration;
use std::thread;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

mod wavetable;
use wavetable::WaveTable;

mod wavetype;
use wavetype::WaveType;

mod virtual_codes;
use virtual_codes::VIRTUAL_CODES;

pub fn is_key_pressed(key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_pressed_for_code(*key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_pressed_for_code(key: i32) -> bool {
	unsafe { (windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(key) as u16) & 0x8000 != 0 }
}

pub fn is_key_released(key: i32) -> bool {
	unsafe { windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(key) & 0x0001 != 0 }
}




fn main() {
    println!("Press 1 for Sin wave, 2 for Saw wave, etc...");

    let mut is_playing = false;
    let mut stop_playing = false;
    let mut start_playing = false;
    
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let base_frequency = 110.0;
    let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
    
    //let kb_layout = "ZSXCFVGBNJMK,l./";
    let kb_layout = "zsxcfvgbnjmk,l./";
    let mut key_pressed = false;
    let mut current_key = usize::MAX;

    'program_loop: loop {
        let mut frequency = 0.0;

        key_pressed = false;

        if is_key_pressed('0'){
            break 'program_loop;
        }

        'char_loop: for i in 0..16{
            if let Some(key) = kb_layout.chars().nth(i) {
                if is_key_pressed(key){
                    key_pressed = true;
                    
                    if current_key != i {
                        //stop_playing = true;
                        frequency = base_frequency * twelfth_root_of_two.powf(i as f32);
                        if !is_playing {
                            start_playing = true;
                            stop_playing = false;
                        }
                    }

                    current_key = i;

                    break 'char_loop;
                }

            }
        }

        if !key_pressed {
            current_key = usize::MAX;
            frequency = 0.0;
            if !stop_playing {
                stop_playing = true;
            }
        }
        
        if is_playing && stop_playing {
            sink.stop();
            start_playing = false;
            is_playing = false;
        }
        
        if !is_playing && start_playing {
            let mut oscillator = WavetableOscillator::new(44100, 64, WaveType::Sine);
            oscillator.set_frequency(frequency);
            oscillator.set_gain(-30.0);

            sink.append(oscillator);
            sink.play();
            is_playing = true;
            start_playing = false;
        }  
    }
    
}

