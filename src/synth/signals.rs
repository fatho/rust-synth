use std;

#[derive(Debug, Clone)]
pub struct Add<S1, S2>(pub S1, pub S2);

#[derive(Debug, Clone)]
pub struct Mul<S1, S2>(pub S1, pub S2);

#[derive(Debug, Clone)]
pub struct Map<S, F>(pub S, pub F);

pub trait SignalGenerator {
    type Frame;

    fn set_sampling_parameters(&mut self, samples_per_second: f32);
    fn next(&mut self) -> Self::Frame;
    fn reset(&mut self);

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

    fn map<F, NewFrame>(self, fun: F) -> Map<Self, F> where
        F: FnMut(Self::Frame) -> NewFrame,
        Self: Sized
    {
        Map(self, fun)
    }
}

impl SignalGenerator for f32 {
    type Frame = f32;

    fn set_sampling_parameters(&mut self, _samples_per_second: f32) {
    }

    fn next(&mut self) -> f32 {
        *self
    }

    fn reset(&mut self) {}
}

impl<'a, Signal> SignalGenerator for &'a mut Signal where
    Signal: 'a + SignalGenerator
{
    type Frame = Signal::Frame;

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        (*self).set_sampling_parameters(samples_per_second);
    }

    fn next(&mut self) -> Self::Frame {
        (*self).next()
    }

    fn reset(&mut self) {
        (*self).reset()
    }
}

impl<S1: SignalGenerator, S2: SignalGenerator> SignalGenerator for Add<S1, S2> where
    S1::Frame: std::ops::Add<S2::Frame>
{
    type Frame = <S1::Frame as std::ops::Add<S2::Frame>>::Output;

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        self.0.set_sampling_parameters(samples_per_second);
        self.1.set_sampling_parameters(samples_per_second);
    }

    fn next(&mut self) -> Self::Frame {
        self.0.next() + self.1.next()
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<S1: SignalGenerator, S2: SignalGenerator> SignalGenerator for Mul<S1, S2> where
    S1::Frame: std::ops::Mul<S2::Frame>
{
    type Frame = <S1::Frame as std::ops::Mul<S2::Frame>>::Output;

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        self.0.set_sampling_parameters(samples_per_second);
        self.1.set_sampling_parameters(samples_per_second);
    }

    fn next(&mut self) -> Self::Frame {
        self.0.next() * self.1.next()
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<S, F, B> SignalGenerator for Map<S, F> where
    S: SignalGenerator,
    F: FnMut(S::Frame) -> B,
{
    type Frame = B;

    fn set_sampling_parameters(&mut self, samples_per_second: f32) {
        self.0.set_sampling_parameters(samples_per_second);
    }

    fn next(&mut self) -> Self::Frame {
        self.1(self.0.next())
    }

    fn reset(&mut self) {
        self.0.reset();
    }
}

pub struct SignalIterator<Signal>(pub Signal);

impl<Signal: SignalGenerator> Iterator for SignalIterator<Signal> {
    type Item = Signal::Frame;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (std::usize::MAX, None)
    }
}
