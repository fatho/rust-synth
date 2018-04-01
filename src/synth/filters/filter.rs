use std;

pub struct Id<S>(std::marker::PhantomData<S>);

pub struct Compose<F1, F2>(pub F1, pub F2);

pub struct Split<F>(pub F);

pub fn id<S>() -> Id<S> {
    Id(std::marker::PhantomData)
}

pub trait Filter {
    type Input;
    type Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output;

    fn chain<F>(self, next: F) -> Compose<Self, F> where
        Self: Sized
    {
        Compose(self, next)
    }
}

impl<S> Filter for Id<S> {
    type Input = S;
    type Output = S;

    fn filter(&mut self, input: S) -> S {
        input
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

impl<'a, F> Filter for &'a mut F where
    F: 'a + Filter
{
    type Input = F::Input;
    type Output = F::Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        (*self).filter(input)
    }
}
