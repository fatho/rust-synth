use std;

use synth::foundation::{SoundModule, Parameter, SamplingParameters};
use synth::foundation::SignalGenerator;
use synth::foundation::Filter;

/// A piece of equipment that has some parameters set through automation.
#[derive(Debug, Clone)]
pub struct Automated<E, C> {
    equipment: E,
    controller: C
}

impl<E, C: Controller<E>> Automated<E,C> {
    pub fn with_generated_param<P, G>(self, param: P, generator: G) -> Automated<E, ChainController<C, GeneratorController<P, G>>> where
        P: Parameter<Target=E>,
        G: SignalGenerator<Output=P::Value>
    {
        Automated {
            equipment: self.equipment,
            controller: self.controller.then(
                GeneratorController {
                    param: param,
                    value_gen: generator
                }
            )
        }
    }

}

pub trait AutomationExt: SoundModule {
    fn automated(self) -> Automated<Self, NopController<Self>> where
        Self: Sized
    {
        Automated {
            equipment: self,
            controller: NopController(std::marker::PhantomData)
        }
    }
}

impl<E: SoundModule> AutomationExt for E {}

impl<E, C> SoundModule for Automated<E, C> where
    E: SoundModule,
    C: SoundModule
{
    fn reset(&mut self) {
        self.equipment.reset();
        self.controller.reset();
    }

    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.equipment.set_sampling_parameters(params);
        self.controller.set_sampling_parameters(params);
    }
}

impl<E, C> Filter for Automated<E, C> where
    E: Filter,
    C: Controller<E>
{
    type Input = E::Input;
    type Output = E::Output;

    fn filter(&mut self, input: Self::Input) -> Self::Output {
        self.controller.update(&mut self.equipment);
        self.equipment.filter(input)
    }
}

impl<E, C> SignalGenerator for Automated<E, C> where
    E: SignalGenerator,
    C: Controller<E>
{
    type Output = E::Output;

    fn next(&mut self) -> Self::Output {
        self.controller.update(&mut self.equipment);
        self.equipment.next()
    }
}

/// A controller modifies a piece of equipment.
pub trait Controller<E>: SoundModule {
    fn update(&mut self, &mut E);
}

pub trait ControllerExt<E>: Controller<E> {
    fn then<C>(self, next: C) -> ChainController<Self, C> where
        Self: Sized,
        C: Controller<E>
    {
        ChainController(self, next)
    }
}

impl<E, C: Controller<E>> ControllerExt<E> for C {}

#[derive(Debug, Clone)]
pub struct ChainController<C1, C2>(pub C1, pub C2);

impl<C1, C2> SoundModule for ChainController<C1, C2> where
    C1: SoundModule,
    C2: SoundModule
{
    fn reset(&mut self) {
        self.0.reset();
        self.1.reset();
    }

    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.0.set_sampling_parameters(params);
        self.1.set_sampling_parameters(params);
    }
}

impl<C1, C2, E> Controller<E> for ChainController<C1, C2> where
    C1: Controller<E>,
    C2: Controller<E>
{
    fn update(&mut self, target: &mut E) {
        self.0.update(target);
        self.1.update(target);
    }
}

/// Controls a parameter through a generator.
#[derive(Debug, Clone)]
pub struct GeneratorController<Param, Gen> {
    param: Param,
    value_gen: Gen
}

impl<P, G> SoundModule for GeneratorController<P, G> where
    G: SoundModule
{
    fn reset(&mut self) {
        self.value_gen.reset();
    }

    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {
        self.value_gen.set_sampling_parameters(params);
    }
}

impl<P, S> Controller<P::Target> for GeneratorController<P, S> where
    P: Parameter,
    S: SignalGenerator<Output=P::Value>
{
    fn update(&mut self, target: &mut P::Target) {
        self.param.set(target, self.value_gen.next())
    }
}


#[derive(Debug, Clone)]
pub struct NopController<E>(std::marker::PhantomData<E>);


impl<E> SoundModule for NopController<E> {
    fn reset(&mut self) {}

    fn set_sampling_parameters(&mut self, params: &SamplingParameters) {}
}

impl<E> Controller<E> for NopController<E> {
    fn update(&mut self, target: &mut E) {}
}
