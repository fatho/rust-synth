use std;

use synth::foundation::Sample;

fn clamp<A>(value: A, min: A, max: A) -> A where
    A: Copy + PartialOrd
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[test]
fn test_clamp() {
    assert_eq!(0.0, clamp(0.0, -1.0, 1.0));
    assert_eq!(1.0, clamp(2.0, -1.0, 1.0));
    assert_eq!(-1.0, clamp(-2.0, -1.0, 1.0));
    assert!(clamp(std::f32::NAN, -1.0, 1.0).is_nan());
}

pub fn hard_limit<S: Sample>(input: S) -> S {
    clamp(input, S::lower_limit(), S::upper_limit())
}
