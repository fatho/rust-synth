use foundation::{Filter, SignalGenerator, SoundModule, SamplingParameters, Frequency};
use std;

#[derive(Debug, Clone)]
pub struct LowPassRC<Freq> {
    cutoff_frequency: Freq,
    sample_rate: Frequency,
    last_output: f32,
}

impl<Freq> LowPassRC<Freq> where
    Freq: SignalGenerator<Output=Frequency>
{

    pub fn new(cutoff_frequency: Freq) -> Self
    {
        LowPassRC {
            cutoff_frequency: cutoff_frequency,
            sample_rate: Frequency::from_hertz(std::f32::NAN),
            last_output: 0.0,
        }
    }

    fn next_coefficient(&mut self) -> f32 {
        let beta = 2.0 * std::f32::consts::PI * self.cutoff_frequency.next() / self.sample_rate;
        beta / (beta + 1.0)
    }
}

impl<Freq: SoundModule> SoundModule for LowPassRC<Freq> {
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.cutoff_frequency.set_sampling_parameters(params);
        self.sample_rate = params.sample_rate();
    }

    fn reset(&mut self) {
        self.cutoff_frequency.reset();
        self.last_output = 0.0;
    }
}

impl<Freq: SignalGenerator<Output=Frequency>> Filter for LowPassRC<Freq> {
    type Input = f32;
    type Output = f32;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let alpha = self.next_coefficient();
        let current = self.last_output * (1. - alpha) + input * alpha;
        self.last_output = current;
        current
    }
}
