use std;

/// Parameters that influence how sound is generated and communicated between
/// different pieces of equipment.
pub struct SamplingParameters {
    /// The number of samples per second.
    pub samples_per_second: f32
}

impl SamplingParameters {
    pub fn audio_cd() -> SamplingParameters {
        Self::with_rate(44100.0)
    }

    /// Construct sampling parameters for a given sampling rate measured in Hz.
    pub fn with_rate(sample_rate: f32) -> SamplingParameters {
        SamplingParameters {
            samples_per_second: sample_rate
        }
    }

    /// Return the current sample rate.
    pub fn sample_rate(&self) -> f32 {
        self.samples_per_second
    }

    /// Return the Nyquist frequency associated with these parameters. This is
    /// the maximum audio frequency that can be represented with the given
    /// sample rate.
    pub fn nyquist_rate(&self) -> f32 {
        self.samples_per_second / 2.0
    }
}


/// A general trait for all things that are considered part of a synthesizer.
pub trait SoundModule {
    /// Reset the internal state of the piece of equipment (e.g. transient state
    /// that is (usually) not directly set by the user).
    fn reset(&mut self);

    /// Set the sampling parameters used for the whole setup. This ensures that
    /// all pieces of equipment agree on how to communicate audio signals.
    fn set_sampling_parameters(&mut self, params: &SamplingParameters);

    // fn save(&self, ...);
    // fn load(...) -> Self;
}

/// A parameter of a piece of equipment can be changed through automation.
pub trait Parameter {
    type Target;
    type Value;

    fn set(&self, target: &mut Self::Target, value: Self::Value);
}

/// A parameter that refers to the sound module as a whole.
pub struct SelfParameter<T>(std::marker::PhantomData<T>);

impl<T> Parameter for SelfParameter<T> {
    type Target = T;
    type Value = T;

    fn set(&self, target: &mut Self::Target, value: Self::Value) {
        std::mem::replace(target, value);
    }
}

/// Treating constant values as a sound module can be useful.
impl SoundModule for f32 {
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}


impl<'a, E> SoundModule for &'a mut E where
    E: 'a + SoundModule
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        (*self).set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        (*self).reset()
    }
}