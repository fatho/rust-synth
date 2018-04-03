use std;

use synth::module::{SoundModule, SamplingParameters};
use synth::signals::SignalGenerator;
use synth::waveform::{Waveform, Saw, Sine, Rect, Triangle};

pub fn sine<F>(frequency: F) -> Oscillator<Sine, F> {
    Oscillator::new(frequency, Sine)
}

pub fn saw<F>(frequency: F) -> Oscillator<Saw, F> {
    Oscillator::new(frequency, Saw)
}

pub fn triangle<F>(frequency: F) -> Oscillator<Triangle, F> {
    Oscillator::new(frequency, Triangle)
}

pub fn square<F>(frequency: F) -> Oscillator<Rect, F> {
    Oscillator::new(frequency, Rect(0.5))
}

pub fn rect<F>(duty_cycle: f32, frequency: F) -> Oscillator<Rect, F> {
    Oscillator::new(frequency, Rect(duty_cycle))
}

#[derive(Debug, Clone)]
pub struct Oscillator<Shape, Freq> {
    phase: f32,
    frequency: Freq,
    shape: Shape,
    samples_per_second: f32
}

impl<Shape, Freq> Oscillator<Shape, Freq> {
    pub fn new(frequency: Freq, shape: Shape) -> Self {
        Oscillator {
            phase: 0.0f32,
            frequency: frequency,
            shape: shape,
            samples_per_second: std::f32::NAN
        }
    }
}

impl<Shape, Freq> SoundModule for Oscillator<Shape, Freq> where
    Freq: SoundModule
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.frequency.set_sampling_parameters(params);
        self.samples_per_second = params.sample_rate();
    }

    fn reset(&mut self) {
        self.frequency.reset();
        self.phase = 0.0;
    }
}

impl<Shape, Freq> SignalGenerator for Oscillator<Shape, Freq> where
    Shape: Waveform,
    Freq: SignalGenerator<Output = f32>
{
    type Output = f32;

    fn next(&mut self) -> f32 {
        let value = self.shape.phase_amplitude(self.phase);
        self.phase = (self.phase + self.frequency.next() / self.samples_per_second).fract();
        value
    }
}
