use std;

use synth::equipment::{Equipment, SamplingParameters};
use synth::filters::filter::Filter;
use synth::sample::Sample;

#[derive(Debug, Clone)]
pub struct Delay<S> {
    duration_secs: f32,
    sample_rate: f32,
    delay_buffer: std::vec::Vec<S>,
    buffer_index: usize
}

impl<S> Delay<S> where
    S: Sample
{
    pub fn new(duration_secs: f32) -> Self {
        Delay {
            duration_secs: duration_secs,
            sample_rate: std::f32::NAN,
            delay_buffer: vec![S::equilibrium(); 1],
            buffer_index: 0
        }
    }

    fn reallocate_buffer(&mut self) {
        let num_samples = (self.sample_rate * self.duration_secs) as usize;
        self.delay_buffer.resize(num_samples.max(1), S::equilibrium());
        self.buffer_index = self.buffer_index % self.delay_buffer.len();
    }
}

impl<S> Equipment for Delay<S> where
    S: Sample
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.sample_rate = params.sample_rate();
        self.reallocate_buffer();
    }

    fn reset(&mut self) {
        self.buffer_index = 0;
        for x in self.delay_buffer.iter_mut() {
            *x = S::equilibrium()
        }
    }
}

impl<S> Filter for Delay<S> where
    S: Sample
{
    type Input = S;
    type Output = S;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let delayed = std::mem::replace(&mut self.delay_buffer[self.buffer_index], input);
        self.buffer_index = (self.buffer_index + 1) % self.delay_buffer.len();
        delayed
    }
}
