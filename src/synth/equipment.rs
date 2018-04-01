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
pub trait Equipment {
    /// Reset the internal state of the piece of equipment (e.g. transient state
    /// that is (usually) not directly set by the user).
    fn reset(&mut self);

    /// Set the sampling parameters used for the whole setup. This ensures that
    /// all pieces of equipment agree on how to communicate audio signals.
    fn set_sampling_parameters(&mut self, params: &SamplingParameters);

    // fn save(&self, ...);
    // fn load(...) -> Self;
}


/// Treating constant values as a piece of equipment can be useful.
impl Equipment for f32 {
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}


impl<'a, E> Equipment for &'a mut E where
    E: 'a + Equipment
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        (*self).set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        (*self).reset()
    }
}
