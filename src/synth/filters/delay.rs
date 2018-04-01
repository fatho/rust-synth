use std;

use synth::equipment::{Equipment, SamplingParameters};
use synth::filters::filter::Filter;
use synth::sample::Sample;

#[derive(Debug, Clone)]
struct RingBuffer<S> {
    buffer: std::vec::Vec<S>,
    index: usize
}

impl<S> RingBuffer<S> where
    S: Sample
{
    fn new(size: usize) -> Self {
        RingBuffer {
            buffer: vec![S::equilibrium(); size],
            index: 0
        }
    }

    fn resize(&mut self, new_size: usize) {
        self.buffer.resize(new_size, S::equilibrium());
        self.index = self.index % self.buffer.len();
    }

    fn current_mut(&mut self) -> &mut S {
        &mut self.buffer[self.index]
    }

    fn forward(&mut self) {
        self.index = (self.index + 1) % self.buffer.len();
    }

    fn shift(&mut self, in_value: S) -> S {
        let out_value = std::mem::replace(self.current_mut(), in_value);
        self.forward();
        out_value
    }

    fn reset(&mut self) {
        self.index = 0;
        for x in self.buffer.iter_mut() {
            *x = S::equilibrium()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Delay<S> {
    duration_secs: f32,
    sample_rate: f32,
    delay_buffer: RingBuffer<S>
}

impl<S> Delay<S> where
    S: Sample
{
    pub fn new(duration_secs: f32) -> Self {
        Delay {
            duration_secs: duration_secs,
            sample_rate: std::f32::NAN,
            delay_buffer: RingBuffer::new(1),
        }
    }

    fn reallocate_buffer(&mut self) {
        let num_samples = (self.sample_rate * self.duration_secs) as usize;
        self.delay_buffer.resize(num_samples.max(1));
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
        self.delay_buffer.reset()
    }
}

impl<S> Filter for Delay<S> where
    S: Sample
{
    type Input = S;
    type Output = S;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.delay_buffer.shift(input)
    }
}

#[derive(Debug, Clone)]
pub struct Echo<S> {
    duration_secs: f32,
    dampening: f32,
    sample_rate: f32,
    delay_buffer: RingBuffer<S>
}

impl<S> Echo<S> where
    S: Sample
{
    pub fn new(duration_secs: f32, dampening: f32) -> Self {
        Echo {
            duration_secs: duration_secs,
            dampening: dampening,
            sample_rate: std::f32::NAN,
            delay_buffer: RingBuffer::new(1),
        }
    }

    fn reallocate_buffer(&mut self) {
        let num_samples = (self.sample_rate * self.duration_secs) as usize;
        self.delay_buffer.resize(num_samples.max(1));
    }
}

impl<S> Equipment for Echo<S> where
    S: Sample
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.sample_rate = params.sample_rate();
        self.reallocate_buffer();
    }

    fn reset(&mut self) {
        self.delay_buffer.reset()
    }
}

impl<S> Filter for Echo<S> where
    S: Sample + std::ops::Mul<f32, Output=S> + std::ops::Add<Output=S>
{
    type Input = S;
    type Output = S;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let current = *self.delay_buffer.current_mut();
        *self.delay_buffer.current_mut() = (current + input) * self.dampening;
        self.delay_buffer.forward();
        current
    }
}
