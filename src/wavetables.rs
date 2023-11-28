
use std::ops::Index;
use std::f32::consts::PI;

use lazy_static::lazy_static;

use crate::wavetype::WaveType;

lazy_static! {
    pub static ref WAVE_TABLES: WaveTables = WaveTables::new();
}

const WAVE_TABLE_SAMPLES: usize = 128;

pub struct WaveTables {
    sin: WaveTable,
    saw: WaveTable,
    tri: WaveTable,
    square: WaveTable,
    pulse: WaveTable,
}

impl WaveTables {
    pub fn new() -> WaveTables{
        WaveTables { 
            sin: WaveTable::new(WAVE_TABLE_SAMPLES, WaveType::Sin), 
            saw: WaveTable::new(WAVE_TABLE_SAMPLES, WaveType::Saw), 
            tri: WaveTable::new(WAVE_TABLE_SAMPLES, WaveType::Triangle), 
            square: WaveTable::new(WAVE_TABLE_SAMPLES, WaveType::Square), 
            pulse: WaveTable::new(WAVE_TABLE_SAMPLES, WaveType::Pulse), 
        }
    }

    pub fn get_wave_table(&self, wave_type: &WaveType) -> &WaveTable{
        return match wave_type {
            WaveType::Sin => &self.sin,
            WaveType::Saw => &self.saw,
            WaveType::Triangle => &self.tri,
            WaveType::Square => &self.square,
            WaveType::Pulse => &self.pulse,
        };
    }
}

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
        let mut table = WaveTable { 
            wave_type: wave_type, 
            wave_table: Vec::with_capacity(wave_table_size), 
            wave_table_size: wave_table_size };

        table.populate_wave_table();
        return table;
    }

    fn populate_wave_table(&mut self){
        self.wave_table.clear();
        let wave_table_size = self.wave_table_size;
        let wave_table = &mut self.wave_table;

        for n in 0..wave_table_size {
            let t = n as f32 / wave_table_size as f32;

            let sin_value = (2.0 * PI * t).sin();

            match self.wave_type {
                WaveType::Sin => wave_table.push(sin_value),
                WaveType::Saw => wave_table.push(((t + PI) / PI) % 2.0 - 1.0),
                WaveType::Triangle => wave_table.push(sin_value.asin() * (2.0 / PI)),
                WaveType::Square => wave_table.push(sin_value.signum()),
                WaveType::Pulse => {
                    let duty_cycle = 0.2;
                    if t < duty_cycle {
                        wave_table.push(1.0)
                    } else {
                        wave_table.push(-1.0)
                    }
                }
            }
        }
    }
}