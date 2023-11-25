use std::sync::mpsc::*;

use egui::*;
use rodio::{OutputStream, Sink};
use eframe::{run_native, App, NativeOptions, egui};

#[macro_use]
extern crate strum_macros;
use strum::IntoEnumIterator;

mod time;
mod envelope;
mod wavetable;
mod keyboard;
mod virtual_codes;
mod general_oscillator;
mod wavetable_oscillator;

mod wavetype;
use wavetype::WaveType;

mod synthesizer;
use synthesizer::{Synthesizer, SynthEvent};

struct OxidizerApp {
    current_notes: Vec<i32>,
    new_notes: Vec<i32>,
    selected_wave_type: WaveType,

    synth_sender: Sender<SynthEvent>
}

impl OxidizerApp{
    fn default(sender: Sender<SynthEvent>) -> Self {
        Self { 
            current_notes: Vec::new(),
            new_notes: Vec::new(),
            selected_wave_type: WaveType::default(),

            synth_sender: sender
        }
    }

    fn handle_input(&mut self, ctx: &eframe::egui::Context){
        self.new_notes.clear();

        let kb_layout = "zsxcfvgbnjmk";
        for i in 0..16 {
            if let Some(key) = kb_layout.chars().nth(i) {
                if ctx.input(|input| keyboard::is_key_pressed(input, key) || keyboard::is_key_down(input, key)) {
                    self.new_notes.push(i as i32);
                }
            }
        }

        for i in &self.new_notes {
            if !self.current_notes.contains(i) {
                let _ = self.synth_sender.send(SynthEvent::NotePress(*i));               
            }
        }   

        for i in &self.current_notes {
            if !self.new_notes.contains(i){
                let _ = self.synth_sender.send(SynthEvent::NoteRelease(*i)); 
            }
        }

        self.current_notes = self.new_notes.clone();
    }

    fn render(&mut self, ui: &mut Ui){
        ui.heading("Press any of these keys to make noise: zsxcfvgbnjmk");

        ui.separator();

        
        ui.horizontal(|ui| {
            ui.label("Wave Form:");

            for wave_type in WaveType::iter(){
                let display_str: &'static str = wave_type.clone().into();
                if ui.selectable_value(&mut self.selected_wave_type, wave_type.clone(), display_str).changed() {
                    let _ = self.synth_sender.send(SynthEvent::ChangeWaveType(wave_type));
                }
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
        });
    }
}


fn main() -> Result<(), eframe::Error> {

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let (sender, receiver): (Sender<SynthEvent>, Receiver<SynthEvent>) = channel();
    let synth = Synthesizer::new(receiver);

    sink.append(synth);
    sink.play();

    env_logger::init(); 
    let options = NativeOptions::default();
    return run_native(
        "Test App", 
        options, 
        Box::new(|_cc| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = OxidizerApp::default(sender);
            return Box::<OxidizerApp>::new(app);
        }));
    
}