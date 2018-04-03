use synth::module::{SoundModule, SamplingParameters};
use synth::signals::SignalGenerator;
use synth::knob::Knob;

pub mod filter;
pub use filter::*;

pub mod delay;
pub use delay::*;

pub mod distortion;
pub use distortion::*;

pub mod inspection;
pub use inspection::*;

pub mod lowpass;
pub use lowpass::LowPassRC;

/// Convenience trait for constructing a filtered signal generator. It is
/// automatically implemented for all signal generators.
pub trait FilteredExt: SignalGenerator {
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

    /// Continuously adjust the knob according to the output signal of this generator.
    fn frobnicate(self, knob: Knob<Self::Output>) -> Filtered<Self, Frobnicator<Self::Output>> where
        Self::Output: Copy,
        Self: Sized
    {
        self.filtered(Frobnicator::new(knob))
    }
}

/// A filtered signal generator.
#[derive(Debug, Clone)]
pub struct Filtered<S, F> {
    generator: S,
    filter: F,
}

impl<S, F> SoundModule for Filtered<S, F> where
    S: SoundModule, F: SoundModule
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

impl<S: SignalGenerator> FilteredExt for S {}
