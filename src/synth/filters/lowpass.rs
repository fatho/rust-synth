use synth::foundation::{Filter, SoundModule, SamplingParameters, Parameter, Frequency};
use std;

#[derive(Debug, Clone)]
pub struct LowPassRC {
    cutoff_frequency: Frequency,
    sample_rate: Frequency,
    alpha: f32,
    last_output: f32,
}

#[derive(Debug, Clone)]
pub struct CutoffFrequencyParam<Target>(std::marker::PhantomData<Target>);

impl Parameter for CutoffFrequencyParam<LowPassRC> {
    type Target = LowPassRC;
    type Value = Frequency;

    fn set(&self, target: &mut Self::Target, value: Self::Value) {
        target.set_cutoff_frequency(value);
    }
}


impl LowPassRC {

    pub fn new(cutoff_frequency: Frequency) -> Self {
        LowPassRC {
            cutoff_frequency: cutoff_frequency,
            sample_rate: Frequency::from_hertz(std::f32::NAN),
            last_output: 0.0,
            alpha: std::f32::NAN
        }
    }

    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: Frequency) {
        self.cutoff_frequency = cutoff_frequency;
        self.recompute_coefficient();
    }

    fn recompute_coefficient(&mut self) {
        let beta = 2.0 * std::f32::consts::PI * self.cutoff_frequency / self.sample_rate;
        self.alpha = beta / (beta + 1.0)
    }

    pub fn cutoff_frequency() -> CutoffFrequencyParam<LowPassRC> {
        CutoffFrequencyParam(std::marker::PhantomData)
    }
}

impl SoundModule for LowPassRC {
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.sample_rate = params.sample_rate();
        self.recompute_coefficient();
    }

    fn reset(&mut self) {
        self.last_output = 0.0;
    }
}

impl Filter for LowPassRC {
    type Input = f32;
    type Output = f32;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let current = self.last_output * (1. - self.alpha) + input * self.alpha;
        self.last_output = current;
        current
    }
}
