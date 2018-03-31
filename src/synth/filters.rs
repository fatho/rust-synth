use synth::signals::SignalGenerator;

use std;


pub trait Filterable {
    fn low_pass_rc<C>(self, cutoff_frequency: C) -> LowPassRC<Self, C> where
        Self: Sized,
        C: SignalGenerator;
}

impl<S: SignalGenerator> Filterable for S {
    fn low_pass_rc<C>(self, cutoff_frequency: C) -> LowPassRC<Self, C> {
        LowPassRC::new(self, cutoff_frequency)
    }
}

#[derive(Debug, Clone)]
pub struct LowPassRC<S, C> {
    cutoff_frequency: C,
    sample_rate: f32,
    last_output: f32,
    input: S
}

impl<S, C> LowPassRC<S, C> {

    pub fn new(input: S, cutoff_frequency: C) -> Self {
        LowPassRC {
            cutoff_frequency: cutoff_frequency,
            sample_rate: std::f32::NAN,
            last_output: 0.0,
            input: input
        }
    }

    fn compute_coefficient(&self, cutoff_frequency: f32) -> f32 {
        let beta = 2.0 * std::f32::consts::PI * cutoff_frequency / self.sample_rate;
        beta / (beta + 1.0)
    }
}

impl<S, C> SignalGenerator for LowPassRC<S, C> where
    S: SignalGenerator<Frame=f32>,
    C: SignalGenerator<Frame=f32>,
{
    type Frame = f32;

    fn reset(&mut self) {
        self.last_output = 0.0;
    }

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        self.input.set_sampling_parameters(samples_per_second);
        self.cutoff_frequency.set_sampling_parameters(samples_per_second);
        self.sample_rate = samples_per_second;
    }

    fn next(&mut self) -> Self::Frame {
        let cutoff = self.cutoff_frequency.next();
        let alpha = self.compute_coefficient(cutoff);
        let current = self.last_output * (1. - alpha) + self.input.next() * alpha;
        self.last_output = current;
        current
    }
}
