use synth::foundation::{Filter, SoundModule, SamplingParameters};
use synth::knob::Knob;

/// A frobnicator is a filter that leaves a signal unchanged, but uses it to set
/// the value of a knob.
#[derive(Debug)]
pub struct Frobnicator<T: Copy> {
    knob: Knob<T>
}

impl<T: Copy> Frobnicator<T> {
    pub fn new(knob: Knob<T>) -> Self {
        Frobnicator {
            knob: knob
        }
    }
}

impl<T: Copy> SoundModule for Frobnicator<T> {
    fn reset(&mut self) {}

    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}
}

impl<T: Copy> Filter for Frobnicator<T> {
    type Input = T;
    type Output = T;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.knob.set(input);
        input
    }
}
