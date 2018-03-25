use std;

use synth::signals::SignalGenerator;
use synth::waveform::Waveform;

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

impl<Shape, Freq> SignalGenerator for Oscillator<Shape, Freq> where
    Shape: Waveform,
    Freq: SignalGenerator<Frame = f32>
{
    type Frame = f32;

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        self.frequency.set_sampling_parameters(samples_per_second);
        self.samples_per_second = samples_per_second
    }

    fn next(&mut self) -> f32 {
        let value = self.shape.phase_amplitude(self.phase);
        self.phase = (self.phase + self.frequency.next() / self.samples_per_second).fract();
        value
    }

    fn reset(&mut self) {
        self.frequency.reset();
        self.phase = 0.0;
    }
}
