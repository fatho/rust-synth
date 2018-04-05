use std;
use foundation::{Sample, SamplingParameters, SoundModule, Filter};
use data::SlidingWindowMax;

pub struct LookaheadLimiter {
    lookahead_count: usize,
    buffer: SlidingWindowMax<Magnitude>
}

impl LookaheadLimiter {
    pub fn new(lookahead_count: usize) -> Self {
        let mut limiter = LookaheadLimiter {
            lookahead_count: lookahead_count,
            buffer: SlidingWindowMax::with_capacity(lookahead_count)
        };
        limiter.reset();
        limiter
    }
}

impl SoundModule for LookaheadLimiter {
    fn reset(&mut self) {
        self.buffer.clear();
        std::iter::repeat(<Self as Filter>::Input::equilibrium())
            .take(self.lookahead_count)
            .for_each(|x| self.buffer.enqueue(Magnitude(x)));
    }

    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}
}

impl Filter for LookaheadLimiter {
    type Input = f32;
    type Output = Self::Input;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let max_amplitude = self.buffer.maximum().map(|mag| mag.0.abs()).unwrap_or(1.0).max(1.0);
        self.buffer.enqueue(Magnitude(input));
        self.buffer.dequeue().unwrap().0 / max_amplitude
    }
}

/// Floats ordered by absolute value, NaN is smallest.
#[derive(Debug, Copy, Clone)]
struct Magnitude(f32);

impl PartialEq for Magnitude {
    fn eq(&self, other: &Self) -> bool {
        self.0.abs() == other.0.abs() || (self.0.is_nan() && other.0.is_nan())
    }
}

impl Eq for Magnitude {}

impl PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Magnitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.abs().partial_cmp(&(*other).0.abs()) {
            Some(ordering) => ordering,
            None => match (self.0.is_nan(), other.0.is_nan()) {
                (true, false) => std::cmp::Ordering::Less,
                (true, true) => std::cmp::Ordering::Equal,
                (false, true) => std::cmp::Ordering::Greater,
                _ => panic!("BUG! should not happen")
            }
        }
    }
}

#[test]
fn test_magnitude_order() {
    assert_eq!(Magnitude(-1.), Magnitude(1.));
    assert_eq!(Magnitude(std::f32::NAN), Magnitude(std::f32::NAN));
    assert_eq!(Magnitude(std::f32::NEG_INFINITY), Magnitude(std::f32::INFINITY));
    assert!(Magnitude(1.1) < Magnitude(-1.2));
    assert!(Magnitude(1.1) != Magnitude(1.2));
    assert!(Magnitude(std::f32::NAN) < Magnitude(-1.2));
    assert!(Magnitude(std::f32::NAN) < Magnitude(std::f32::NEG_INFINITY));
}

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
    use std;

    assert_eq!(0.0, clamp(0.0, -1.0, 1.0));
    assert_eq!(1.0, clamp(2.0, -1.0, 1.0));
    assert_eq!(-1.0, clamp(-2.0, -1.0, 1.0));
    assert!(clamp(std::f32::NAN, -1.0, 1.0).is_nan());
}

pub fn hard_limit<S: Sample>(input: S) -> S {
    clamp(input, S::lower_limit(), S::upper_limit())
}
