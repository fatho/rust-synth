use synth::equipment::{Equipment, Parameter, SamplingParameters};
use synth::signals::SignalGenerator;
use synth::filters::Filter;

/// A piece of equipment that has some parameters set through automation.
#[derive(Debug, Clone)]
pub struct Automated<E, C> {
    equipment: E,
    controller: C
}

pub trait AutomationExt: Equipment {
    fn automated<C>(self, controller: C) -> Automated<Self, C> where
        Self: Sized
    {
        Automated {
            equipment: self,
            controller: controller
        }
    }
}

impl<E: Equipment> AutomationExt for E {}

impl<E, C> Equipment for Automated<E, C> where
    E: Equipment,
    C: Equipment
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

pub fn generate_param<P, G>(param: P, generator: G) -> GeneratorController<P, G> where
    P: Parameter,
    G: SignalGenerator<Output=P::Value>
{
    GeneratorController {
        param: param,
        value_gen: generator
    }
}

/// A controller modifies a piece of equipment.
pub trait Controller<E>: Equipment {
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

impl<C1, C2> Equipment for ChainController<C1, C2> where
    C1: Equipment,
    C2: Equipment
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

impl<P, G> Equipment for GeneratorController<P, G> where
    G: Equipment
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
