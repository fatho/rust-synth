#[macro_use]
extern crate criterion;
extern crate synth;

use criterion::{Fun, Criterion};

use synth::oscillator::{saw, sine, triangle, square};
use synth::foundation::module::{SamplingParameters, SoundModule};
use synth::foundation::generator::{SignalIterator, Const, constant};
use synth::foundation::types::Frequency;
use synth::oscillator::Oscillator;
use synth::waveform::{Waveform, Saw, Sine, Rect, Triangle};

// fn fract_bench(x: f32) -> f32 {
//     x.fract()
// }

// fn fmod_bench(x: f32) -> f32 {
//     x % 1.
// }

// fn floor_sub_bench(x: f32) -> f32 {
//     x - x.floor()
// }

// fn fract_vs_fmod(c: &mut Criterion) {
//     let fun_fract = Fun::new("fract", |b, i| b.iter(|| fract_bench(*i)));
//     let fun_fmod = Fun::new("fmod", |b, i| b.iter(|| fmod_bench(*i)));
//     let fun_floor = Fun::new("floor", |b, i| b.iter(|| floor_sub_bench(*i)));
//     let functions = vec!(fun_fract, fun_fmod, fun_floor);
//     c.bench_functions("fractional", functions, 1.0125125);
// }

pub struct BaselineWaveform;

impl Waveform for BaselineWaveform {
    fn phase_amplitude(&self, phase: f32) -> f32 {
        phase
    }
}

fn make_test_osc<W: Waveform>(shape: W) -> Oscillator<W, Const<Frequency>> {
    let a1 = Frequency::from_hertz(440.);
    let mut osc = Oscillator::new(constant(a1), shape);
    osc.set_sampling_parameters(&SamplingParameters::audio_cd());
    osc
}

fn oscillator_bench(c: &mut Criterion) {
    let num_samples = 44100;

    c.bench_function("baseline", move |b| b.iter(|| SignalIterator(make_test_osc(BaselineWaveform)).take(num_samples).count()));
    c.bench_function("saw", move |b| b.iter(|| SignalIterator(make_test_osc(Saw)).take(num_samples).count()));
    c.bench_function("sine", move |b| b.iter(|| SignalIterator(make_test_osc(Sine)).take(num_samples).count()));
    c.bench_function("square", move |b| b.iter(|| SignalIterator(make_test_osc(Rect(0.5))).take(num_samples).count()));
    c.bench_function("triangle", move |b| b.iter(|| SignalIterator(make_test_osc(Triangle)).take(num_samples).count()));
}

criterion_group!(oscillators, oscillator_bench);
criterion_main!(oscillators);
