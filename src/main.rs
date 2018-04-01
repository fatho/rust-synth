extern crate byteorder;
extern crate rand;

use byteorder::{WriteBytesExt,LittleEndian};

pub mod synth;
use synth::automation::*;
use synth::equipment::*;
use synth::oscillator::*;
use synth::signals::*;
use synth::noise::*;
use synth::filters::*;
use synth::sample;

fn main() {
    let sampling_params = SamplingParameters::audio_cd();
    let half_tone: f32 = 2.0f32.powf(1. / 12.);
    let a1: f32 = 440.;
    let c2: f32 = a1 * half_tone.powi(3);
    let e2: f32 = a1 * half_tone.powi(7);

    let ampl = saw(0.1).map(|x| (1. - x) * 1.0);
    let lfo_freq = saw(0.1).add(1.).map(|x| x.powi(2)).mul(2.);
    let lfo = sine(lfo_freq).map(|x| e2 * 2.0f32.powf((1. - x) * 3.0));
    let mut gen = triangle(a1).mul(0.3)
        .add(triangle(c2).mul(0.3))
        .add(triangle(e2).mul(0.3))
        .add(white_noise().mul(0.05))
        .filtered(LowPassRC::new(440.0)
                  .automated()
                  .with_generated_param(LowPassRC::cutoff_frequency(), lfo))
        .filtered(Echo::new(0.5, 0.7))
        .mul(ampl)
    ;

    gen.set_sampling_parameters(&sampling_params);
    eprintln!("{:?}", gen);

    let mut out = std::io::stdout();

    SignalIterator(&mut gen)
        .map(sample::hard_limit)
        .map(sample::Resample::resample)
        .take(sampling_params.sample_rate() as usize * 10)
        .for_each(|value| out.write_i16::<LittleEndian>(value).unwrap());

    eprintln!("{:?}", gen);
}
