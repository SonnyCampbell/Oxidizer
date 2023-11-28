use std::sync::mpsc::*;

use egui::*;
use rodio::{OutputStream, Sink};
use eframe::{run_native, App, NativeOptions, egui};

#[macro_use]
extern crate strum_macros;
use strum::{EnumCount, IntoEnumIterator};

mod time;
mod envelope;
mod keyboard;
mod virtual_codes;
mod oscillator;
mod note_generator;
mod sound_generator;


mod constants;
use constants::*;

mod wavetype;
use wavetype::WaveType;

mod synthesizer;
use synthesizer::{Synthesizer, SynthEvent, EnvelopeParam};


struct OxidizerApp {
    current_notes: Vec<i32>,
    new_notes: Vec<i32>,
    sound_gen_oscillators: [SoundGenOscParams; OscNumber::COUNT],
    attack: f32,
    decay: f32,
    release: f32,

    synth_sender: Sender<SynthEvent>
}

impl OxidizerApp{
    fn default(sender: Sender<SynthEvent>) -> Self {
        Self { 
            current_notes: Vec::new(),
            new_notes: Vec::new(),
            sound_gen_oscillators: SoundGenOscParams::create_default_array(),
            attack: 1.0,
            decay: 1.0,
            release: 2.0,

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

    fn render_grid(&mut self, ui: &mut Ui){

        for osc_params in &mut self.sound_gen_oscillators {
            let display_num = osc_params.num as i32 + 1;
            ui.label(format!("Oscillator {display_num} Wave Form:"));

            ui.horizontal(|ui| {
                for wave_type in WaveType::iter(){
                    let display_str: &'static str = wave_type.into();
                    if ui.selectable_value(&mut osc_params.wave_type, wave_type, display_str).changed() {
                        osc_params.enabled = true;
                        let _ = self.synth_sender.send(SynthEvent::ChangeSoundGenOscParams(osc_params.clone()));
                    }
                }
            });
            ui.end_row();
        }

        ui.label("Attack:");
        let slider = Slider::new(&mut self.attack, 0.0..=32.0)
            .logarithmic(true)
            .smallest_positive(0.001)
            .smart_aim(false)
            .min_decimals(1);
        
        if ui.add(slider).changed() {
            let _ = self.synth_sender.send(SynthEvent::ChangeEnvelope(EnvelopeParam::AttackTime, self.attack));
        }
        ui.end_row();

        ui.label("Decay:");
        let slider = Slider::new(&mut self.decay, 0.0..=32.0)
            .logarithmic(true)
            .smallest_positive(0.001)
            .smart_aim(false)
            .min_decimals(1);
        
        if ui.add(slider).changed() {
            let _ = self.synth_sender.send(SynthEvent::ChangeEnvelope(EnvelopeParam::DecayTime, self.decay));
        }
        ui.end_row();

        ui.label("Release:");
        let slider = Slider::new(&mut self.release, 0.0..=32.0)
            .logarithmic(true)
            .smallest_positive(0.001)
            .smart_aim(false)
            .min_decimals(1);
        
        if ui.add(slider).changed() {
            let _ = self.synth_sender.send(SynthEvent::ChangeEnvelope(EnvelopeParam::ReleaseTime, self.release));
        }
        ui.end_row();


    }

    fn render(&mut self, ui: &mut Ui){
        ui.heading("Press any of these keys to make noise: zsxcfvgbnjmk");

        ui.separator();

        Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                self.render_grid(ui);
            });
        
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