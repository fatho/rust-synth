use super::filter::Filter;
use synth::equipment::{Equipment, SamplingParameters};
use std;

#[derive(Debug, Clone)]
pub struct LowPassRC {
    cutoff_frequency: f32,
    sample_rate: f32,
    alpha: f32,
    last_output: f32,
}

impl LowPassRC {

    pub fn new(cutoff_frequency: f32) -> Self {
        LowPassRC {
            cutoff_frequency: cutoff_frequency,
            sample_rate: std::f32::NAN,
            last_output: 0.0,
            alpha: std::f32::NAN
        }
    }

    fn recompute_coefficient(&mut self) {
        let beta = 2.0 * std::f32::consts::PI * self.cutoff_frequency / self.sample_rate;
        self.alpha = beta / (beta + 1.0)
    }
}

impl Equipment for LowPassRC {
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
