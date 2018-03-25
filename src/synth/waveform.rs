use std;

pub trait Waveform {
    fn phase_amplitude(&self, phase: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct Sine;

#[derive(Debug, Clone)]
pub struct Saw;

impl Waveform for Sine {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        (2.0 * std::f32::consts::PI * phase.fract()).sin()
    }
}

impl Waveform for Saw {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        1.0 - 2.0 * phase % 1.0
    }
}
