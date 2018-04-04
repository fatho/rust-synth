#[macro_use]
extern crate criterion;

use criterion::{Fun, Criterion};

fn fract_bench(x: f32) -> f32 {
    x.fract()
}

fn fmod_bench(x: f32) -> f32 {
    x % 1.
}

fn floor_sub_bench(x: f32) -> f32 {
    x - x.floor()
}

fn fract_vs_fmod(c: &mut Criterion) {
    let fun_fract = Fun::new("fract", |b, i| b.iter(|| fract_bench(*i)));
    let fun_fmod = Fun::new("fmod", |b, i| b.iter(|| fmod_bench(*i)));
    let fun_floor = Fun::new("floor", |b, i| b.iter(|| floor_sub_bench(*i)));
    let functions = vec!(fun_fract, fun_fmod, fun_floor);
    c.bench_functions("fractional", functions, 1.0125125);
}

fn oscillator_bench(c: &mut Criterion) {
    
}

criterion_group!(benches, fract_vs_fmod);
criterion_main!(benches);
