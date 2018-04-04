//! A module for filters that distort a signal.

use foundation::Filter;
use foundation::{SoundModule, SamplingParameters};
use super::limiter::{hard_limit};

#[derive(Debug, Clone)]
pub struct Overdrive {
    factor: f32
}

impl Overdrive {

    pub fn new(factor: f32) -> Self {
        Overdrive {
            factor: factor
        }
    }
}

impl SoundModule for Overdrive
{
    fn reset(&mut self) {}

    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}
}

impl Filter for Overdrive
{
    type Input = f32;
    type Output = f32;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        hard_limit(input * self.factor)
    }
}
