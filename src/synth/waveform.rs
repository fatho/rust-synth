use std;

pub trait Waveform {
    fn phase_amplitude(&self, phase: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct Sine;

#[derive(Debug, Clone)]
pub struct Saw;

#[derive(Debug, Clone)]
pub struct Triangle;

#[derive(Debug, Clone)]
/// A rectangular wave with the given duty cycle.
pub struct Rect(pub f32);

#[derive(Debug, Clone)]
pub struct Wavetable<T>(pub T);

impl Waveform for Sine {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        (2.0 * std::f32::consts::PI * phase.fract()).sin()
    }
}

impl Waveform for Saw {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        2.0 * (phase % 1.0) - 1.0
    }
}

impl Waveform for Triangle {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        1.0 - 4.0 * (phase - 0.5).abs()
    }
}

impl Waveform for Rect {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        if phase < self.0 {
            -1.0
        } else if phase < self.0 * 2.0 {
            1.0
        } else {
            0.0
        }
    }
}

impl<T> Waveform for Wavetable<T> where
    T: std::ops::Deref<Target = [f32]>
{
    fn phase_amplitude(&self, phase: f32) -> f32 {
        let length = self.0.len();
        let index = phase * length as f32;
        let index1 = index.floor() as usize % length;
        let index2 = index.ceil() as usize % length;
        let interp = index.fract();
        (1.0 - interp) * self.0[index1] + interp * self.0[index2]
    }
}
