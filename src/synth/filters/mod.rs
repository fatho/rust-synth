use synth::equipment::{Equipment, SamplingParameters};
use synth::signals::SignalGenerator;

pub mod filter;
pub use filter::*;

pub mod lowpass;
pub use lowpass::LowPassRC;

/// A filtered signal generator.
#[derive(Debug, Clone)]
pub struct Filtered<S, F> {
    generator: S,
    filter: F,
}

impl<S, F> Equipment for Filtered<S, F> where
    S: Equipment, F: Equipment
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.filter.set_sampling_parameters(params);
        self.generator.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.filter.reset();
        self.generator.reset();
    }
}

impl<S, F> SignalGenerator for Filtered<S, F> where
    S: SignalGenerator,
    F: Filter<Input=S::Output>
{
    type Output = F::Output;

    fn next(&mut self) -> Self::Output {
        self.filter.filter(self.generator.next())
    }
}

pub trait FilterExt: SignalGenerator {
    fn filtered<F>(self, filter: F) -> Filtered<Self, F> where
        Self: Sized,
        F: Filter
    {
        Filtered {
            generator: self,
            filter: filter
        }
    }

    fn map<F, O>(self, fun: F) -> Filtered<Self, Map<F, Self::Output>> where
        F: Fn(Self::Output) -> O,
        Self: Sized
    {
        self.filtered(lift(fun))
    }
}

impl<S: SignalGenerator> FilterExt for S {}
