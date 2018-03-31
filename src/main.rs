extern crate byteorder;
extern crate rand;

use byteorder::{WriteBytesExt,LittleEndian};

pub mod synth;
use synth::oscillator::*;
use synth::signals::*;
use synth::waveform::*;
use synth::noise::*;
use synth::filters::*;
use synth::sample;

fn main() {
    const SAMPLE_RATE: usize = 44100;
    let half_tone: f32 = 2.0f32.powf(1. / 12.);
    let a1: f32 = 440.;
    let c2: f32 = a1 * half_tone.powi(3);
    let e2: f32 = a1 * half_tone.powi(7);

    let lfo = Oscillator::new(2.0, Sine).map(|x| e2 * 2.0f32.powf((1. - x) * 3.0));
    let mut gen = Oscillator::new(a1, Saw).mul(0.3)
        .add(Oscillator::new(c2, Saw).mul(0.3))
        .add(Oscillator::new(e2, Saw).mul(0.3))
        .add(white_noise().mul(0.1))
        .low_pass_rc(lfo)
    ;

    gen.set_sampling_parameters(SAMPLE_RATE as f32);
    eprintln!("{:?}", gen);

    let mut out = std::io::stdout();

    SignalIterator(&mut gen)
        .map(sample::hard_limit)
        .map(sample::Resample::resample)
        .take(SAMPLE_RATE * 10)
        .for_each(|value| out.write_i16::<LittleEndian>(value).unwrap());

}
