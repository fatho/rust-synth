use std;
use super::equipment::{Equipment, SamplingParameters};

#[derive(Debug, Clone)]
pub struct Add<S1, S2>(pub S1, pub S2);

#[derive(Debug, Clone)]
pub struct Mul<S1, S2>(pub S1, pub S2);

pub trait SignalGenerator: Equipment {
    type Output;

    fn next(&mut self) -> Self::Output;

    fn add<S>(self, other: S) -> Add<Self, S> where
        S: SignalGenerator,
        Self: Sized
    {
        Add(self, other)
    }

    fn mul<S>(self, other: S) -> Mul<Self, S> where
        S: SignalGenerator,
        Self: Sized
    {
        Mul(self, other)
    }
}

impl SignalGenerator for f32 {
    type Output = f32;

    fn next(&mut self) -> f32 {
        *self
    }
}

impl<'a, Signal> SignalGenerator for &'a mut Signal where
    Signal: 'a + SignalGenerator
{
    type Output = Signal::Output;

    fn next(&mut self) -> Self::Output {
        (*self).next()
    }
}

impl<S1: Equipment, S2: Equipment> Equipment for Add<S1, S2> {
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.0.set_sampling_parameters(params);
        self.1.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<S1: SignalGenerator, S2: SignalGenerator> SignalGenerator for Add<S1, S2> where
    S1::Output: std::ops::Add<S2::Output>
{
    type Output = <S1::Output as std::ops::Add<S2::Output>>::Output;

    fn next(&mut self) -> Self::Output {
        self.0.next() + self.1.next()
    }
}


impl<S1: Equipment, S2: Equipment> Equipment for Mul<S1, S2> {
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.0.set_sampling_parameters(params);
        self.1.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<S1: SignalGenerator, S2: SignalGenerator> SignalGenerator for Mul<S1, S2> where
    S1::Output: std::ops::Mul<S2::Output>
{
    type Output = <S1::Output as std::ops::Mul<S2::Output>>::Output;

    fn next(&mut self) -> Self::Output {
        self.0.next() * self.1.next()
    }
}

pub struct SignalIterator<Signal>(pub Signal);

impl<Signal: SignalGenerator> Iterator for SignalIterator<Signal> {
    type Item = Signal::Output;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (std::usize::MAX, None)
    }
}
