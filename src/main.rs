use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::mpsc::*;

use egui::*;
use egui_plot::{Line, Plot, PlotPoints};
use rodio::{OutputStream, Sink};
use eframe::{run_native, App, NativeOptions, egui};

slint::include_modules!();

use slint::{VecModel, ModelRc, SharedString};
use strum::{EnumCount, IntoEnumIterator, VariantNames};

use oxidizer::wavetables::*;
use oxidizer::constants::*;
use oxidizer::wavetype::WaveType;
use oxidizer::synthesizer::{Synthesizer, SynthEvent, EnvelopeParam};

struct OxidizerApp {
    current_notes: Vec<i32>,
    new_notes: Vec<i32>,
    sound_gen_oscillators: [SoundGenOscParams; OscNumber::COUNT],
    attack: f32,
    decay: f32,
    release: f32,
    lfo: LfoParams,

    synth_sender: Sender<SynthEvent>
}

impl OxidizerApp{
    fn default(sender: Sender<SynthEvent>) -> Self {
        Self { 
            current_notes: Vec::new(),
            new_notes: Vec::new(),
            sound_gen_oscillators: SoundGenOscParams::create_default_array(),
            attack: 0.1,
            decay: 1.0,
            release: 0.1,
            lfo: Default::default(),
            synth_sender: sender
        }
    }

    fn handle_input(&mut self, ctx: &eframe::egui::Context){
        self.new_notes.clear();

        let kb_layout = "zsxcfvgbnjmk";
        for i in 0..16 {
            if let Some(key) = kb_layout.chars().nth(i) {
                if ctx.input(|input| 
                        oxidizer::keyboard::is_key_pressed(input, key) || 
                        oxidizer::keyboard::is_key_down(input, key)) {
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

    fn plot_oscillator(ui: &mut Ui, name: String, wave_type: &WaveType) -> egui::Response {

        let table = WAVE_TABLES.get_wave_table(wave_type);
        let n = table.wave_table_size;
        let points: PlotPoints = (0..(2 * n))
            .map(|i| {
                [i as f64 / n as f64, table[i % n] as f64 ]
            })
            .collect();

        let line = Line::new(points);
        

        Plot::new(name)
            .height(40.0)
            .width(100.0)
            .show_axes(false)
            //.data_aspect(1.0)
            .auto_bounds_x()
            .auto_bounds_y()
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .allow_scroll(false)
            .allow_drag(false)
            .show_x(false)
            .show_y(false)
            .show(ui, |plot_ui| plot_ui.line(line))
            .response
    }

    fn render_lfo(&mut self, ui: &mut Ui){

        if ui.checkbox(&mut self.lfo.enabled, format!("LFO")).changed() {
            let _ = self.synth_sender.send(SynthEvent::ChangeLfoParams(self.lfo.clone()));
        };
        ui.end_row();

        ui.add_enabled_ui(self.lfo.enabled, |panel| {
            
            panel.label("Wave Form:");
        });

        ui.add_enabled_ui(self.lfo.enabled, |panel| {
            panel.horizontal(|ui| {
                for wave_type in WaveType::iter(){
                    let display_str: &'static str = wave_type.into();
                    if ui.selectable_value(&mut self.lfo.wave_type, wave_type, display_str).changed() {
                        let _ = self.synth_sender.send(SynthEvent::ChangeLfoParams(self.lfo.clone()));
                    }
                }
            });
        });

        ui.end_row();

        if self.lfo.enabled {
            Self::plot_oscillator(ui, format!("LFO Wave Form"), &self.lfo.wave_type);

            ui.end_row();

            ui.label("LFO Frequency:");
            let slider = Slider::new(&mut self.lfo.frequency, 0.0..=10.0)
                .fixed_decimals(1);
            
            if ui.add(slider).changed() {
                let _ = self.synth_sender.send(SynthEvent::ChangeLfoParams(self.lfo.clone()));
            }
            ui.end_row();
        }

        ui.separator();
        ui.end_row();
    }

    fn render_envelope(&mut self, ui: &mut Ui){

        ui.label(RichText::new("Envelope").underline());
        ui.end_row();
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

        ui.separator();
        ui.end_row();
    }

    fn render_oscillators(&mut self, ui: &mut Ui){
        for osc_params in &mut self.sound_gen_oscillators {
            let display_num = osc_params.num as i32 + 1;

            if ui.checkbox(&mut osc_params.enabled, format!("Oscillator {display_num}")).changed() {
                let _ = self.synth_sender.send(SynthEvent::ChangeSoundGenOscParams(osc_params.clone()));
            };
            ui.end_row();

            ui.add_enabled_ui(osc_params.enabled, |panel| {
                
                panel.label("Wave Form:");
            });

            ui.add_enabled_ui(osc_params.enabled, |panel| {
                panel.horizontal(|ui| {
                    for wave_type in WaveType::iter(){
                        let display_str: &'static str = wave_type.into();
                        if ui.selectable_value(&mut osc_params.wave_type, wave_type, display_str).changed() {
                            let _ = self.synth_sender.send(SynthEvent::ChangeSoundGenOscParams(osc_params.clone()));
                        }
                    }
                });
            });

            ui.end_row();

            if osc_params.enabled {

                ui.label("Unisons:");
                ui.group(|ui| {
                    let slider = Slider::new(&mut osc_params.unisons, 1..=16)
                        .integer()
                        .custom_formatter(|n, _| {
                            format!("{n}v")
                        });
                    
                    if ui.add(slider).changed() {
                        let _ = self.synth_sender.send(SynthEvent::ChangeSoundGenOscParams(osc_params.clone()));
                    }

                    let slider = Slider::new(&mut osc_params.unison_detune_pct, 0.0..=1.0)
                        .fixed_decimals(2)
                        .custom_formatter(|n, _| {
                            let i = (n * 100.0).round() as i64;
                            format!("{i}%")
                        });
                    
                    if ui.add(slider).changed() {
                        let _ = self.synth_sender.send(SynthEvent::ChangeSoundGenOscParams(osc_params.clone()));
                    }
                });
                ui.end_row();

                Self::plot_oscillator(ui, format!("Oscillator {display_num} Wave Form"), &osc_params.wave_type);
                ui.end_row();
            }

            ui.separator();
            ui.end_row();
        }
    }

    fn render_grid(&mut self, ui: &mut Ui){

        self.render_oscillators(ui);
        self.render_envelope(ui);
        self.render_lfo(ui);

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

            if ctx.input(|input| oxidizer::keyboard::is_key_pressed_for_code(input, Key::Escape)) {
                //Todo: quit app
            }

            self.handle_input(ctx);
        });
    }
}


fn main() {

    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let (ui_sender, synth_receiver): (Sender<SynthEvent>, Receiver<SynthEvent>) = channel();

    let synth = Synthesizer::new(synth_receiver);

    sink.append(synth);
    sink.play();

    // env_logger::init(); 
    // let options = NativeOptions::default();
    // run_native(
    //     "Test App", 
    //     options, 
    //     Box::new(|_cc| {
    //         // This gives us image support:
    //         //egui_extras::install_image_loaders(&cc.egui_ctx);
    //         let app = OxidizerApp::default(ui_sender);
    //         return Box::new(app);
    //     }));

        

    let window = MainWindow::new().unwrap();

    let app = Rc::new(RefCell::new(OxidizerApp::default(ui_sender)));

    let wave_types: Vec<SharedString> = WaveType::VARIANTS
        .iter()
        .map(|i| {
            (*i).to_string().into()
        }).collect();
    window.set_osc_wave_types(ModelRc::from(Rc::new(VecModel::from(wave_types))));
    
    let clone = app.clone();
    window.global::<KeyPress>().on_key_pressed(move |value| {
        let _ = clone.borrow().synth_sender.send(SynthEvent::NotePress(value.parse::<i32>().unwrap().clone()));  
    });

    let clone = app.clone();
    window.global::<KeyPress>().on_key_released(move |value| {
        let _ = clone.borrow().synth_sender.send(SynthEvent::NoteRelease(value.parse::<i32>().unwrap().clone())); 
    });

    let clone = app.clone();
    window.global::<KeyPress>().on_selected_wave_form(move |index, opt| {
        
        if let Ok(wave_type) = WaveType::from_str(&opt) {
            let app = &mut clone.borrow_mut();

            let params = &mut app.sound_gen_oscillators[index as usize];
            params.wave_type = wave_type;

            let event = SynthEvent::ChangeSoundGenOscParams(params.clone());
            let _ = app.synth_sender.send(event);
        }
    });

    let clone = app.clone();
    window.global::<KeyPress>().on_changed_unison_voices(move |index, value| {
        let app = &mut clone.borrow_mut();
        
        let params = &mut app.sound_gen_oscillators[index as usize];
        params.unisons = value;

        let event = SynthEvent::ChangeSoundGenOscParams(params.clone());
        let _ = app.synth_sender.send(event);
        
    });

    let clone = app.clone();
    window.global::<KeyPress>().on_changed_unison_detune_pct(move |index, value| {
        let app = &mut clone.borrow_mut();
        
        let params = &mut app.sound_gen_oscillators[index as usize];
        params.unison_detune_pct = value as f32 / 100.0;

        let event = SynthEvent::ChangeSoundGenOscParams(params.clone());
        let _ = app.synth_sender.send(event);
        
    });

    let clone = app.clone();
    window.global::<KeyPress>().on_osc_enable_toggled(move |index| {
        let app = &mut clone.borrow_mut();
        
        let params = &mut app.sound_gen_oscillators[index as usize];
        params.enabled = !params.enabled;

        let event = SynthEvent::ChangeSoundGenOscParams(params.clone());
        let _ = app.synth_sender.send(event);
        
    });

    window.run().unwrap();
    
    
}