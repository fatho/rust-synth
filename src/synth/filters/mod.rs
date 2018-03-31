use synth::signals::SignalGenerator;

pub mod lowpass;
pub use lowpass::LowPassRC;

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

