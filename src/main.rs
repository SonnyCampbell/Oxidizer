//use rodio::buffer::SamplesBuffer;
//use rodio::Sink;
use rodio::OutputStream;
use std::f32::consts::PI;
use std::time::Duration;
use rodio::Source;

struct WavetableOscillator{
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    gain: f32,
    amplitude: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        let gain = 0.0;

        return WavetableOscillator { 
            sample_rate: sample_rate, 
            wave_table: wave_table, 
            index: 0.0, 
            index_increment: 0.0,
            gain: gain,
            amplitude: Self::calculate_amplitude(gain)
        };
    }

    fn calculate_amplitude(gain: f32) -> f32 {
        return (10.0 as f32).powf(gain / 20.0);
    }

    fn set_frequency(&mut self, frequency: f32){
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn set_gain(&mut self, gain: f32){
        self.gain = gain;
        self.amplitude = Self::calculate_amplitude(gain);
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample * self.amplitude;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        // if self.index is 1.25 we do
        // wave_table[truncated_index] * 0.75 + wave_table[next_index] * 0.25
        // because the index is nearer to the truncated_index than the next_index

        return truncated_index_weight * self.wave_table[truncated_index] + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32>{
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator{
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    let wave_table_size: usize = 64; //why is this 64?
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    // fill wavetable with values of sine wave.
    // change this to fill wave table with values of selected wave type
    for n in 0..wave_table_size {
        let t = n as f32 / wave_table_size as f32;

        // sine wave
        //wave_table.push((2.0 * PI * t).sin());
        
        // saw wave
        //wave_table.push(((t + PI) / PI) % 2.0 - 1.0);

        // triangle wave
        //wave_table.push(1.0 - (t - 0.5).abs()*4.0)

        // square wave
        wave_table.push((2.0 * PI * t).sin().signum());

        // pulse wave
        let duty_cycle = 0.2;
        if t % 1.0 < duty_cycle {
            wave_table.push(1.0)
        } else {
            wave_table.push(-1.0)
        }
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(220.0);
    oscillator.set_gain(-20.0);

    // Set up the audio output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");
    
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(3));
}