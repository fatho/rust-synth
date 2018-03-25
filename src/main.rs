extern crate byteorder;

pub mod synth;

fn main() {
    println!("Hello, world!");

    //   buffered::buf_test();
    unbuffered::unbuf_test();
}

mod unbuffered {
    use std;
    use byteorder::{WriteBytesExt,LittleEndian};

    use synth::oscillator::*;
    use synth::signals::*;
    use synth::waveform::*;
    use synth::sample;

    pub fn unbuf_test() {
        const SAMPLE_RATE: usize = 48000;

        let lfo = 440.0.add(40.0.mul(Oscillator::new(1.0, Sine)));
        let sine1 = Oscillator::new(lfo.clone(), Sine);
        let sine2 = Oscillator::new(lfo.clone().mul(0.5), Sine);
        let saw1 = Oscillator::new(lfo.mul(0.5), Saw);
        let mut gen = sine1.mul(0.3)
            .add(sine2.mul(0.3))
            .add(saw1.mul(0.3));

        gen.set_sampling_parameters(SAMPLE_RATE as f32);

        let mut out = std::io::BufWriter::new(std::fs::File::create("/tmp/unbuf.raw").unwrap());

        SignalIterator(gen)
            .map(sample::hard_limiter)
            .map(sample::resampler)
            .take(SAMPLE_RATE * 60)
            .for_each(|value| out.write_i16::<LittleEndian>(value).unwrap());
    }
}
