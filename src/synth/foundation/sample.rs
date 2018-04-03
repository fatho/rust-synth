use std;

pub trait Sample: Copy + PartialOrd {
    fn lower_limit() -> Self;
    fn upper_limit() -> Self;
    fn equilibrium() -> Self;
}

impl Sample for f32 {
    fn lower_limit() -> Self { -1.0 }
    fn upper_limit() -> Self { 1.0 }
    fn equilibrium() -> Self { 0.0 }
}

impl Sample for i16 {
    fn lower_limit() -> Self { -std::i16::MAX }
    fn upper_limit() -> Self { std::i16::MAX }
    fn equilibrium() -> Self { 0 }
}

pub trait Resample<To> {
    fn resample(self) -> To;
}

impl Resample<i16> for f32 {
    fn resample(self) -> i16 {
        (self * std::i16::MAX as f32) as i16
    }
}


#[cfg(test)]
macro_rules! test_resample_impl {
    ($from: ident, $to: ident) => ({
        assert_eq!(<$from as Sample>::lower_limit().resample(), <$to as Sample>::lower_limit());
        assert_eq!(<$from as Sample>::upper_limit().resample(), <$to as Sample>::upper_limit());
        assert_eq!(<$from as Sample>::equilibrium().resample(), <$to as Sample>::equilibrium());
    })
}

#[test]
fn test_resample() {
    test_resample_impl!(f32, i16)
}

