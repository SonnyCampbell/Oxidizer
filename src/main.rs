use rodio::{OutputStream, Sink};
use windows;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

mod wavetable;

mod wavetype;
use wavetype::WaveType;

mod virtual_codes;
use virtual_codes::VIRTUAL_CODES;

mod combined_oscillator;
use combined_oscillator::CombinedOscillator;

use eframe::{run_native, App, NativeOptions, egui};
use egui::*;

pub fn is_key_pressed(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_pressed_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_released(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_released_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_down(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_down_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_pressed_for_code(i: &InputState, key: Key) -> bool {
	i.key_pressed(key)
}

pub fn is_key_released_for_code(i: &InputState, key: Key) -> bool {
	i.key_released(key)
}

pub fn is_key_down_for_code(i: &InputState, key: Key) -> bool {
	i.key_down(key)
}


struct OxidizerApp{
    current_keys: Vec<i32>,
    frequencies: Vec<f32>,
    start_playing: bool,
    stop_playing: bool,
    sink: Sink
}

impl OxidizerApp {
    fn default(sink: Sink) -> Self {
        
        Self { 
            current_keys: Vec::with_capacity(16) ,
            frequencies: Vec::with_capacity(8),
            start_playing: false,
            stop_playing: false,
            sink: sink
        }
    }
}

impl App for OxidizerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("The Heading");

            if ctx.input(|input| input.key_pressed(Key::Escape)) {
                //Todo: quit app
            }

            self.stop_playing = false;
            let mut new_keys: Vec<i32> = Vec::with_capacity(16);
            let base_frequency = 110.0;
            let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
            self.frequencies.clear();

            let kb_layout = "zsxcfvgbnjmk";
            for i in 0..16 {
                if let Some(key) = kb_layout.chars().nth(i) {
                    if ctx.input(|input| is_key_pressed(input, key) || is_key_down(input, key)) {
                        new_keys.push(i as i32);
                        self.frequencies.push(base_frequency * twelfth_root_of_two.powf(i as f32));
                    }
                    if ctx.input(|input| is_key_down(input, key)) {
                        // if !new_keys.contains(&(i as i32)) {
                        //     new_keys.push(i as i32);   
                        // }
                    }
                    if ctx.input(|input| is_key_released(input, key)) {
                        //println!("{} was release", key);
                    }
                }
            }

            if self.current_keys.len() != new_keys.len() {
                println!("nequla");
                self.start_playing = true;
            }
            else{
                for i in &self.current_keys {
                    if !new_keys.contains(&i) {
                        self.start_playing= true;
                    }
                }
            }
            self.current_keys = new_keys.clone();

            if new_keys.len() == 0 {
                //println!("no keys");
                self.stop_playing = true;
                self.current_keys.clear();
            }
            
            if self.stop_playing {
                //println!("stop playing");
                self.sink.stop();
                self.sink.clear();
                self.start_playing = false;
            }

            if self.start_playing {
                println!("start_playing {:?}", self.current_keys);
                println!("combining {} freqs", self.frequencies.len());
    
                let mut combined = CombinedOscillator::new();
                for freq in &self.frequencies {
                    let mut oscillator = WavetableOscillator::new(44100, 64, WaveType::Sine);
                    oscillator.set_frequency(*freq);
                    oscillator.set_gain(-30.0);
                    combined.add_oscillator(oscillator);
                }
    
                if self.sink.len() > 0 as usize {
                    self.sink.stop();
                    self.sink.clear();
                }
    
                self.sink.append(combined);
                self.sink.play();
                self.start_playing = false;
            } 
        });
    }
}


fn main() -> Result<(), eframe::Error> {

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();
    //
    env_logger::init(); 
    let options = NativeOptions::default();
    return run_native(
        "Test App", 
        options, 
        Box::new(|cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = OxidizerApp::default(sink);
            return Box::<OxidizerApp>::new(app);
        }));

    /*
    println!("Press 1 for Sin wave, 2 for Saw wave, etc...");

    let mut stop_playing = false;
    let mut start_playing = false;
    
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let base_frequency = 110.0;
    let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
    
    let kb_layout = "zsxcfvgbnjmk,l./";
    let mut key_pressed = false;
    //let mut current_key = usize::MAX;
    let mut current_keys: Vec<i32> = Vec::with_capacity(16);

    'program_loop: loop {

        key_pressed = false;
        stop_playing = false;

        let mut frequencies: Vec<f32> = Vec::with_capacity(8);
        let mut new_keys: Vec<i32> = Vec::with_capacity(16);

        if is_key_pressed('0'){
            break 'program_loop;
        }

        'char_loop: for i in 0..16{
            if let Some(key) = kb_layout.chars().nth(i) {
                if is_key_pressed(key){
                    
                    key_pressed = true;
                    new_keys.push(i as i32);
                    frequencies.push(base_frequency * twelfth_root_of_two.powf(i as f32));
                }
            }
        }


        if(current_keys.len() != new_keys.len()){
            start_playing = true;
        }
        else{
            for i in current_keys {
                if !new_keys.contains(&i) {
                    start_playing= true;
                }
            }
        }
        current_keys = new_keys.clone();

        

        if !key_pressed {
            stop_playing = true;
            current_keys.clear();
        }
        
        if stop_playing {
            sink.stop();
            sink.clear();
            start_playing = false;
        }
        
        if start_playing {
            println!("start_playing {:?}", current_keys);
            println!("combining {} freqs", frequencies.len());

            let mut combined = CombinedOscillator::new();
            for freq in frequencies {
                let mut oscillator = WavetableOscillator::new(44100, 64, WaveType::Sine);
                oscillator.set_frequency(freq);
                oscillator.set_gain(-30.0);
                combined.add_oscillator(oscillator);
            }

            if sink.len() > 0 as usize {
                sink.stop();
                sink.clear();
            }

            sink.append(combined);
            sink.play();
            start_playing = false;
        }  
    }
     */
    
}


/*
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
*/
