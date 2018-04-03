extern crate byteorder;
extern crate rand;

use byteorder::{WriteBytesExt,LittleEndian};

pub mod synth;
use synth::automation::*;
use synth::foundation::*;
use synth::oscillator::*;
use synth::noise::*;
use synth::filters::*;
use synth::knob::*;
use synth::foundation::types::units as u;

fn main() {
    let sampling_params = SamplingParameters::audio_cd();
    let half_tone: f32 = 2.0f32.powf(1. / 12.);
    let a1 = Frequency::from_hertz(440.);
    let e2 = a1 * half_tone.powi(7);

    let mut freq_knob = Knob::new(440. * u::HZ);
    let noise_amp_knob = Knob::new(0.05);
    let noise_amp = noise_amp_knob.as_generator();

    let ampl = saw(generator::constant(0.1 * u::HZ)).map(|x| (1. - x) * 1.0);
    let lfo_freq = saw(generator::constant(Frequency::from_hertz(0.1))).map(|x| (x + 1.).powi(2) * 2. * u::HZ);
    let lfo = sine(lfo_freq).map(|x| e2 * 2.0f32.powf((1. - x) * 3.0));
    let mut gen = saw(freq_knob.as_generator()).frobnicate(noise_amp_knob).mul(0.3)
        .add(saw(freq_knob.as_generator().mul(half_tone.powi(3))).mul(0.3))
        .add(square(freq_knob.as_generator().mul(half_tone.powi(7)).mul(noise_amp.add(1.))).mul(0.3))
        .add(pink_noise().mul(0.05))
        .filtered(LowPassRC::new(440.0 * u::HZ)
                  .automated()
                  .with_generated_param(LowPassRC::cutoff_frequency(), lfo))
        .filtered(Echo::new(0.5 * units::S, 0.5))
        .mul(ampl)
        ;

    freq_knob.set(a1 / 2.);

    gen.set_sampling_parameters(&sampling_params);
    eprintln!("{:?}", gen);

    let mut out = std::io::stdout();

    generator::SignalIterator(&mut gen)
        .map(hard_limit)
        .map(Resample::resample)
        .take(sampling_params.sample_rate().to_hertz() as usize * 10)
        .for_each(|value| out.write_i16::<LittleEndian>(value).unwrap());

    eprintln!("{:?}", gen);
}

