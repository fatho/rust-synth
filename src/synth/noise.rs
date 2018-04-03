use std;
use std::fmt::Debug;
use rand::{Rng, NewRng, SeedableRng, XorShiftRng};

use synth::foundation::{SignalGenerator, Sample, SoundModule, SamplingParameters};

pub fn white_noise() -> Noise<White, XorShiftRng> {
    Noise::new(White)
}

pub fn pink_noise() -> Noise<Pink, XorShiftRng> {
    Noise::new(Pink::new())
}

pub struct Noise<C, R: SeedableRng> {
    rng: rng::ResettableRng<R>,
    color: C
}

pub trait NoiseColor {
    fn reset(&mut self);
    fn next<R>(&mut self, rng: &mut R) -> f32 where R: Rng;
}

impl<C, R> Debug for Noise<C, R> where
    C: Debug,
    R: Debug + SeedableRng,
    R::Seed: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Noise")
            .field("rng", &self.rng)
            .field("color", &self.color)
            .finish()
    }
}

impl<C, R> Clone for Noise<C, R> where
    C: Clone,
    R: Clone + SeedableRng,
    R::Seed: Clone
{
    fn clone(&self) -> Self {
        Noise {
            rng: self.rng.clone(),
            color: self.color.clone()
        }
    }
}

impl<C: NoiseColor, R: Rng + SeedableRng> SoundModule for Noise<C, R> where
    R::Seed: Clone
{
    fn set_sampling_parameters(&mut self, _: &SamplingParameters) {}

    fn reset(&mut self) {
        self.rng.reset();
        self.color.reset();
    }
}

impl<C: NoiseColor, R: Rng + SeedableRng> SignalGenerator for Noise<C, R> where
    R::Seed: Clone
{
    type Output = f32;

    fn next(&mut self) -> Self::Output {
        self.color.next(&mut self.rng)
    }
}

impl<C, R: NewRng> Noise<C, R> where
    R::Seed: Clone
{
    pub fn new(color: C) -> Self {
        Noise {
            rng: NewRng::new(),
            color: color
        }
    }
}

#[derive(Debug, Clone)]
pub struct White;

impl NoiseColor for White {
    fn reset(&mut self) {}
    fn next<R>(&mut self, rng: &mut R) -> f32 where R: Rng {
        rng.gen_range(<f32 as Sample>::lower_limit(), <f32 as Sample>::upper_limit())
    }
}

#[derive(Debug, Clone)]
pub struct Pink {
    /// The current random values of the octaves
    octaves: [i32; Pink::NUM_OCTAVES],
    /// The number of trailing zeros of the counter determines the octave that
    /// is changed in the current step
    counter: u32,
    /// The current sum of all octaves
    running_sum: i64,
}

impl NoiseColor for Pink {
    fn reset(&mut self) {
        std::mem::replace(self, Pink::new());
    }

    fn next<R>(&mut self, rng: &mut R) -> f32 where R: Rng {
        // Voss-Mc Cartney algorithm for pink noise generation
        // http://www.firstpr.com.au/dsp/pink-noise/#Voss-McCartney
        let cur_octave = (self.counter | Pink::COUNTER_LEADING_MASK).trailing_zeros() as usize;
        assert!(cur_octave < Pink::NUM_OCTAVES);

        let previous = self.octaves[cur_octave];
        let next = rng.gen::<i32>() >> Pink::NOISE_SHIFT;
        self.running_sum = self.running_sum - previous as i64 + next as i64;
        self.octaves[cur_octave] = next;

        self.counter = (self.counter + 1) & Pink::COUNTER_WRAP_MASK;

        (self.running_sum + (rng.gen::<i32>() >> Pink::NOISE_SHIFT) as i64) as f32 * Pink::SCALING
    }
}

impl Pink {
    /// The number of white-noise octaves. Needs a counter of (NUM_OCTAVES - 1) bits.
    const NUM_OCTAVES: usize = 15;
    /// Bit-mask for wrapping the counter.
    const COUNTER_WRAP_MASK: u32 = (1 << (Pink::NUM_OCTAVES - 1)) - 1;
    /// Or-mask applied to counter before counting trailing zeros.
    const COUNTER_LEADING_MASK: u32 = !Pink::COUNTER_WRAP_MASK;
    /// Number of random bits for each sample.
    const NOISE_BITS: usize = 24;
    /// Amount of bits a 32-bit random value needs to be shifted to the right.
    const NOISE_SHIFT: usize = std::mem::size_of::<i32>() * 8 - Pink::NOISE_BITS;
    /// Scaling factor for converting the running sum to a float bounded between
    /// -1 and 1.
    const SCALING: f32 = 1.0 / ((Pink::NUM_OCTAVES + 1) * (1 << (Pink::NOISE_BITS - 1)) as usize) as f32;

    pub fn new() -> Self {
        Pink {
            counter: 0,
            octaves: [0; Pink::NUM_OCTAVES],
            running_sum: 0
        }
    }
}

/// Defines a random number generator that remembers its initial seed. Used for
/// reproducible noise generation even across multiple plays of the same song.
mod rng {
    use rand;
    use rand::{SeedableRng, RngCore};

    #[derive(Debug, Clone)]
    pub struct ResettableRng<R: SeedableRng> {
        base_rng: R,
        initial_seed: R::Seed
    }

    impl<R: SeedableRng + RngCore> RngCore for ResettableRng<R> {
        fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), rand::Error> {
            self.base_rng.try_fill_bytes(bytes)
        }

        fn fill_bytes(&mut self, bytes: &mut [u8]) {
            self.base_rng.fill_bytes(bytes)
        }

        fn next_u32(&mut self) -> u32 {
            self.base_rng.next_u32()
        }

        fn next_u64(&mut self) -> u64 {
            self.base_rng.next_u64()
        }
    }

    impl<R: SeedableRng> SeedableRng for ResettableRng<R> where
        R::Seed: Clone
    {
        type Seed = R::Seed;

        fn from_seed(seed: Self::Seed) -> Self {
            ResettableRng {
                base_rng: R::from_seed(seed.clone()),
                initial_seed: seed
            }
        }
    }

    impl<R: SeedableRng> ResettableRng<R> where
        R::Seed: Clone
    {
        pub fn reset(&mut self) {
            self.base_rng = R::from_seed(self.initial_seed.clone())
        }
    }
}

