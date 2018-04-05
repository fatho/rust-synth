use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Frequency(f32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration(f32);


pub mod units {
    use super::{Frequency, Duration};
    /// Unit Hertz `1/s`
    pub const HZ: Frequency = Frequency(1.);
    /// Unit seconds
    pub const S: Duration = Duration(1.);
}

impl Frequency {
    pub fn from_hertz(hertz: f32) -> Self {
        Frequency(hertz)
    }

    pub fn to_hertz(self) -> f32 {
        self.0
    }
}

impl Duration {
    pub fn from_seconds(seconds: f32) -> Duration {
        Duration(seconds)
    }

    pub fn to_seconds(self) -> f32 {
        self.0
    }
}

impl Div<Duration> for f32 {
    type Output = Frequency;

    #[inline(always)]
    fn div(self, duration: Duration) -> Self::Output {
        Frequency(self / duration.0)
    }
}

impl Div<Frequency> for f32 {
    type Output = Duration;

    #[inline(always)]
    fn div(self, duration: Frequency) -> Self::Output {
        Duration(self / duration.0)
    }
}

impl Mul<Frequency> for Duration {
    type Output = f32;

    #[inline(always)]
    fn mul(self, other: Frequency) -> Self::Output {
        self.0 * other.0
    }
}

impl Mul<Duration> for Frequency {
    type Output = f32;

    #[inline(always)]
    fn mul(self, other: Duration) -> Self::Output {
        self.0 * other.0
    }
}

macro_rules! impl_additive {
    ($type: ident) => {
        impl Add<$type> for $type {
            type Output = $type;

            #[inline(always)]
            fn add(self, other: $type) -> $type {
                $type(self.0 + other.0)
            }
        }

        impl Sub<$type> for $type {
            type Output = $type;

            #[inline(always)]
            fn sub(self, other: $type) -> $type {
                $type(self.0 - other.0)
            }
        }

        impl Neg for $type {
            type Output = $type;

            #[inline(always)]
            fn neg(self) -> $type {
                $type(self.0.neg())
            }
        }
    }
}

macro_rules! impl_scalar_mult {
    ($type: ident) => {
        impl Mul<f32> for $type {
            type Output = $type;

            #[inline(always)]
            fn mul(self, other: f32) -> $type {
                $type(self.0 * other)
            }
        }

        impl Mul<$type> for f32 {
            type Output = $type;

            #[inline(always)]
            fn mul(self, other: $type) -> $type {
                $type(self * other.0)
            }
        }

        impl Div<f32> for $type {
            type Output = $type;

            #[inline(always)]
            fn div(self, other: f32) -> $type {
                $type(self.0 / other)
            }
        }

        impl Div<$type> for $type {
            type Output = f32;

            #[inline(always)]
            fn div(self, other: $type) -> f32 {
                self.0 / other.0
            }
        }
    }
}

impl_additive!(Frequency);
impl_additive!(Duration);

impl_scalar_mult!(Frequency);
impl_scalar_mult!(Duration);
