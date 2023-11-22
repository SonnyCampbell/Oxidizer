
use std::f32::consts::PI;
use std::ops::Index;

use crate::wavetype::WaveType;

pub struct WaveTable {
    wave_type: WaveType,
    wave_table: Vec<f32>,
    pub wave_table_size: usize,
}

//If a mutable value is requested, IndexMut is used instead.
impl Index<usize> for WaveTable {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        return &self.wave_table[i];
    }
}

impl WaveTable {
    pub fn new(wave_table_size: usize, wave_type: WaveType) -> WaveTable {
        return WaveTable { 
            wave_type: wave_type, 
            wave_table: Vec::with_capacity(wave_table_size), 
            wave_table_size: wave_table_size };
    }
    
    pub fn set_wave_type(&mut self, wave_type: WaveType){
        self.wave_type = wave_type;
        self.populate_wave_table()
    }

    fn populate_wave_table(&mut self){
        // fill wavetable with values of sine wave.
        // change this to fill wave table with values of selected wave type
        self.wave_table.clear();
        let wave_table_size = self.wave_table_size;
        let wave_table = &mut self.wave_table;

        for n in 0..wave_table_size {
            let t = n as f32 / wave_table_size as f32;

            match self.wave_type {
                WaveType::Sine => wave_table.push((2.0 * PI * t).sin()),
                WaveType::Saw => wave_table.push(((t + PI) / PI) % 2.0 - 1.0),
                WaveType::Tri => wave_table.push(1.0 - (t - 0.5).abs()*4.0),
                WaveType::Square => wave_table.push((2.0 * PI * t).sin().signum()),
                WaveType::Pulse => {
                    let duty_cycle = 0.2;
                    if t % 1.0 < duty_cycle {
                        wave_table.push(1.0)
                    } else {
                        wave_table.push(-1.0)
                    }
                }
            }
            // sine wave
            //wave_table.push((2.0 * PI * t).sin());
            
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
    }
}
