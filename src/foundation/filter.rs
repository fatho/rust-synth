use std;
use std::fmt::Debug;
use std::marker::PhantomData;

use foundation::{SoundModule, SamplingParameters};

/// The identity filter, returning a signal unchanged.
#[derive(Debug, Clone)]
pub struct Id<S>(PhantomData<S>);

/// Two filters chained together, first applying F1, and then F2 to the result
/// of F1.
#[derive(Debug, Clone)]
pub struct Chain<F1, F2>(pub F1, pub F2);

/// The split filter copies the output of a filter so that it can be processed
/// differently.
#[derive(Debug, Clone)]
pub struct Split<F>(pub F);

/// The map filter, applying an existing function to a signal.
#[derive(Clone)]
pub struct Map<Fun, I>(pub Fun, PhantomData<I>);

/// A "dried" version of a filter that mixes the wet (filtered) signal with the
/// dry (input) signal.
#[derive(Debug, Clone)]
pub struct Dried<F> {
    wet: f32,
    dry: f32,
    filter: F
}

impl<Fun, I> Debug for Map<Fun, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Map").finish()
    }
}

/// Construct the identity filter.
pub fn id<S>() -> Id<S> {
    Id(PhantomData)
}

/// Use a function as a filter.
pub fn lift<F, I, O>(fun: F) -> Map<F, I> where
    F: Fn(I) -> O
{
    Map(fun, PhantomData)
}

pub trait Filter: SoundModule {
    type Input;
    type Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output;

    fn chain<F>(self, next: F) -> Chain<Self, F> where
        Self: Sized
    {
        Chain(self, next)
    }

    fn dry(self, dryness: f32, wetness: f32) -> Dried<Self> where
        Self: Sized
    {
        Dried {
            wet: wetness,
            dry: dryness,
            filter: self
        }
    }
}

impl<S> SoundModule for Id<S> {
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}

impl<S> Filter for Id<S> {
    type Input = S;
    type Output = S;

    #[inline(always)]
    fn filter(&mut self, input: S) -> S {
        input
    }
}

impl<F1,F2> SoundModule for Chain<F1, F2> where
    F1: SoundModule,
    F2: SoundModule
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.0.set_sampling_parameters(params);
        self.1.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }
}

impl<F1, F2> Filter for Chain<F1, F2> where
    F1: Filter,
    F2: Filter<Input=F1::Output>
{
    type Input = F1::Input;
    type Output = F2::Output;

    #[inline(always)]
    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.1.filter(self.0.filter(input))
    }
}

impl<F> SoundModule for Split<F> where
    F: SoundModule
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.0.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.0.reset();
    }
}

impl<F> Filter for Split<F> where
    F: Filter,
    F::Output: Clone
{
    type Input = F::Input;
    type Output = (F::Output, F::Output);

    #[inline(always)]
    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let output = self.0.filter(input);
        (output.clone(), output)
    }
}

impl<F, I> SoundModule for Map<F, I>
{
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}

impl<F, I, O> Filter for Map<F, I> where
    F: Fn(I) -> O
{
    type Input = I;
    type Output = F::Output;

    #[inline(always)]
    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.0(input)
    }
}

impl<F> SoundModule for Dried<F> where
    F: SoundModule
{
    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.filter.set_sampling_parameters(params);
    }

    fn reset(&mut self) {
        self.filter.reset();
    }
}

impl<F> Filter for Dried<F> where
    F: Filter,
    F::Input: std::ops::Mul<f32> + Copy,
    F::Output: std::ops::Mul<f32>,
    <F::Input as std::ops::Mul<f32>>::Output: std::ops::Add<<F::Output as std::ops::Mul<f32>>::Output, Output=F::Output>
{
    type Input = F::Input;
    type Output = F::Output;

    #[inline(always)]
    fn filter(&mut self, input: Self::Input) -> Self::Output {
         input * self.dry + self.filter.filter(input) * self.wet
    }
}

impl<'a, F> Filter for &'a mut F where
    F: 'a + Filter
{
    type Input = F::Input;
    type Output = F::Output;

    #[inline(always)]
    fn filter(&mut self, input: Self::Input) -> Self::Output {
        (*self).filter(input)
    }
}
