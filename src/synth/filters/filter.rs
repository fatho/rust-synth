use std;
use std::fmt::Debug;
use std::marker::PhantomData;

use synth::equipment::{Equipment, SamplingParameters};

/// The identity filter, returning a signal unchanged.
#[derive(Debug, Clone)]
pub struct Id<S>(PhantomData<S>);

/// The compose filter, first applying F1, and then F2 to the result of F1.
#[derive(Debug, Clone)]
pub struct Compose<F1, F2>(pub F1, pub F2);

/// The split filter copies the output of a filter so that it can be processed
/// differently.
#[derive(Debug, Clone)]
pub struct Split<F>(pub F);

/// The map filter, applying an existing function to a signal.
#[derive(Clone)]
pub struct Map<Fun, I>(pub Fun, PhantomData<I>);

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

pub trait Filter: Equipment {
    type Input;
    type Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output;

    fn chain<F>(self, next: F) -> Compose<Self, F> where
        Self: Sized
    {
        Compose(self, next)
    }
}

impl<S> Equipment for Id<S> {
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}

impl<S> Filter for Id<S> {
    type Input = S;
    type Output = S;

    fn filter(&mut self, input: S) -> S {
        input
    }
}

impl<F1,F2> Equipment for Compose<F1, F2> where
    F1: Equipment,
    F2: Equipment
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

impl<F1, F2> Filter for Compose<F1, F2> where
    F1: Filter,
    F2: Filter<Input=F1::Output>
{
    type Input = F1::Input;
    type Output = F2::Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.1.filter(self.0.filter(input))
    }
}

impl<F> Equipment for Split<F> where
    F: Equipment
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

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        let output = self.0.filter(input);
        (output.clone(), output)
    }
}

impl<F, I> Equipment for Map<F, I>
{
    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}

    fn reset(&mut self) {}
}

impl<F, I, O> Filter for Map<F, I> where
    F: Fn(I) -> O
{
    type Input = I;
    type Output = F::Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.0(input)
    }
}

impl<'a, F> Filter for &'a mut F where
    F: 'a + Filter
{
    type Input = F::Input;
    type Output = F::Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        (*self).filter(input)
    }
}
