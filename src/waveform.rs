use std;

pub trait Waveform {
    /// Return the amplitude at the given phase offset in the interval `[0-1)`
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
    #[inline(always)]
    fn phase_amplitude(&self, phase: f32) -> f32 {
        (2.0 * std::f32::consts::PI * phase).sin()
    }
}

impl Waveform for Saw {
    #[inline(always)]
    fn phase_amplitude(&self, phase: f32) -> f32 {
        2.0 * phase - 1.0
    }
}

impl Waveform for Triangle {
    #[inline(always)]
    fn phase_amplitude(&self, phase: f32) -> f32 {
        if phase < 0.25 {
            phase * 4.
        } else if phase < 0.75 {
            2. - phase * 4.
        } else {
            phase * 4. - 4.
        }
    }
}

impl Waveform for Rect {
    #[inline(always)]
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
        let index2 = (index1 + 1) % length;
        let interp = index.fract();
        (1.0 - interp) * self.0[index1] + interp * self.0[index2]
    }
}
