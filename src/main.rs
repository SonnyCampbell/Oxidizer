use rodio::{OutputStream, Sink};

use eframe::{run_native, App, NativeOptions, egui};
use egui::*;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Instant;

mod wavetable_oscillator;
use wavetable_oscillator::WavetableOscillator;

mod wavetable;
use wavetable::WaveTables;

mod virtual_codes;
mod keyboard;
mod time;
mod envelope;

mod wavetype;
use wavetype::WaveType;

mod combined_oscillator;
use combined_oscillator::CombinedOscillator;


mod general_oscillator;
use general_oscillator::GeneralOscillator;


lazy_static! {
    static ref WAVE_TABLES: WaveTables = WaveTables::new();
}

struct OxidizerApp {
    current_notes: HashMap<i32, f32>,
    new_notes: HashMap<i32, f32>,
    //frequencies: Vec<f32>,
    selected_wave_type: WaveType,
    restart_playing: bool,
    sink: Sink,

    just_pressed_notes: Vec<i32>,
    held_notes: Vec<i32>,
    released_notes: Vec<i32>,
    held_oscillators: HashMap<i32, GeneralOscillator>,
    released_oscillators: Vec<GeneralOscillator>,

    wave_tables: &'static WaveTables
}

impl OxidizerApp{
    fn default(sink: Sink, wave_tables: &'static WaveTables) -> Self {
        Self { 
            current_notes: HashMap::new(),
            new_notes: HashMap::new(),
            //frequencies: Vec::with_capacity(8),
            selected_wave_type: WaveType::Sine,
            restart_playing: false,
            sink: sink,

            just_pressed_notes: Vec::with_capacity(16),
            held_notes: Vec::with_capacity(16),
            released_notes: Vec::with_capacity(16),
            held_oscillators: HashMap::new(),
            released_oscillators: Vec::with_capacity(16),

            wave_tables: wave_tables
        }
    }

    fn get_frequency(i: f32) -> f32{
        let base_frequency = 220.0;
        let twelfth_root_of_two = (2.0 as f32).powf(1.0 / 12.0);
        return base_frequency * twelfth_root_of_two.powf(i as f32);
    }

    fn start_stop_playing(&mut self){
        self.just_pressed_notes.clear();
        self.released_notes.clear();
        self.held_notes.clear();

        for i in &self.new_notes {
            if !self.current_notes.contains_key(i.0) {
                self.just_pressed_notes.push(*i.0);
                println!("just pressed {}", time::get_time());
                
            }
            else {
                self.held_notes.push(*i.0);
            }
        }   

        for i in &self.current_notes {
            if !self.new_notes.contains_key(i.0){
                self.released_notes.push(*i.0);
            }
        }

        if self.just_pressed_notes.len() > 0 || self.released_notes.len() > 0 {
            println!("restart");
            self.restart_playing= true;
        }

        self.current_notes = self.new_notes.clone();


        if self.restart_playing {
            let mut combined = CombinedOscillator::new();

            for note in &self.released_notes {
                if let Some(mut removed) = self.held_oscillators.remove(note) {
                    removed.note_released();
                    self.released_oscillators.push(removed);
                }
            }

            for osc in &self.held_oscillators {
                combined.add_oscillator(osc.1.clone());
            }

            let released_copy = self.released_oscillators.clone();
            for i in 0..released_copy.len() {
                if released_copy[i].get_amplitude() <= 0.0 {
                    self.released_oscillators.remove(i);
                }
                else {
                    combined.add_oscillator(released_copy[i].clone());
                }
            }

            for note in &self.just_pressed_notes {
                let freq = self.current_notes[note];
                let wave_table = self.wave_tables.get_wave_table(&self.selected_wave_type);
                let osc = GeneralOscillator::new(freq, 44100, wave_table);
                combined.add_oscillator(osc.clone());
                self.held_oscillators.insert(*note, osc);
            }

            //println!("start_playing {:?}", self.current_notes);
            println!("combining {} osc", combined.len());
            
            // for freq in &self.frequencies {
            //     let wave_table = self.wave_tables.get_wave_table(&self.selected_wave_type);
            //     let mut oscillator = WavetableOscillator::new(44100, wave_table);
            //     oscillator.set_frequency(*freq);
            //     oscillator.set_gain(-10.0);
            //     combined.add_oscillator(oscillator);
            // }

            if self.sink.len() > 0 as usize {
                self.sink.stop();
                self.sink.clear();
            }

            self.sink.append(combined);
            self.sink.play();
            self.restart_playing = false;
        } 
    }

    /*
    fn start_stop_playing1(&mut self){
        if self.current_notes.len() != self.new_notes.len() {
            self.restart_playing = true;
        }
        else{
            for i in &self.current_notes {
                if !self.new_notes.contains(&i) {
                    self.restart_playing= true;
                }
            }
        }
        self.current_notes = self.new_notes.clone();

        if self.new_notes.len() == 0 {
            self.stop_playing = true;
            self.current_notes.clear();
        }
        
        if self.stop_playing {
            self.sink.stop();
            self.sink.clear();
            self.restart_playing = false;
        }

        if self.restart_playing {
            println!("start_playing {:?}", self.current_notes);
            println!("combining {} freqs", self.frequencies.len());

            let mut combined = CombinedOscillator::new();
            for freq in &self.frequencies {
                let wave_table = self.wave_tables.get_wave_table(&self.selected_wave_type);
                let mut oscillator = WavetableOscillator::new(44100, wave_table);
                oscillator.set_frequency(*freq);
                oscillator.set_gain(-10.0);
                combined.add_oscillator(oscillator);
            }

            if self.sink.len() > 0 as usize {
                self.sink.stop();
                self.sink.clear();
            }

            self.sink.append(combined);
            self.sink.play();
            self.restart_playing = false;
        } 
    }
     */

    fn handle_input(&mut self, ctx: &eframe::egui::Context){
        self.new_notes.clear();

        let kb_layout = "zsxcfvgbnjmk";
        for i in 0..16 {
            if let Some(key) = kb_layout.chars().nth(i) {
                if ctx.input(|input| keyboard::is_key_pressed(input, key) || keyboard::is_key_down(input, key)) {
                    self.new_notes.insert(i as i32, Self::get_frequency(i as f32));
                }
            }
        }
    }

    fn render(&mut self, ui: &mut Ui){
        ui.heading("Press any of these keys to make noise: zsxcfvgbnjmk");

        ui.separator();

        
        ui.horizontal(|ui| {
            ui.label("Wave Form:");
            if ui.selectable_value(&mut self.selected_wave_type, WaveType::Sine, "Sin").changed() {
                self.restart_playing = true;
            }
            if ui.selectable_value(&mut self.selected_wave_type, WaveType::Saw, "Saw").changed() {
                self.restart_playing = true;
            }
            if ui.selectable_value(&mut self.selected_wave_type, WaveType::Tri, "Triangle").changed() {
                self.restart_playing = true;
            }
            if ui.selectable_value(&mut self.selected_wave_type, WaveType::Square, "Square").changed() {
                self.restart_playing = true;
            }
            if ui.selectable_value(&mut self.selected_wave_type, WaveType::Pulse, "Pulse").changed() {
                self.restart_playing = true;
            }
        });
        ui.end_row();
    }
}

impl App for OxidizerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.render(ui);

            if ctx.input(|input| keyboard::is_key_pressed_for_code(input, Key::Escape)) {
                //Todo: quit app
            }

            self.handle_input(ctx);

            self.start_stop_playing();
        });
    }
}




fn main() -> Result<(), eframe::Error> {

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    env_logger::init(); 
    let options = NativeOptions::default();
    return run_native(
        "Test App", 
        options, 
        Box::new(|_cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = OxidizerApp::default(sink, &WAVE_TABLES);
            return Box::<OxidizerApp>::new(app);
        }));
    
}