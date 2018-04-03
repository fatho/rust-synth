use std;
use std::cell::Cell;
use std::rc::Rc;

use synth::foundation::{SoundModule, SamplingParameters};
use synth::foundation::SignalGenerator;

/// A knob holds a value that can be used to set parameters in a signal generator.
#[derive(Debug)]
pub struct Knob<T: Copy> {
    value: Rc<Cell<T>>
}

/// A generator getting its value from a knob.
#[derive(Debug, Clone)]
pub struct KnobGenerator<T: Copy> {
    value: Rc<Cell<T>>
}

impl<T: Copy> SoundModule for KnobGenerator<T> {
    fn reset(&mut self) {}

    fn set_sampling_parameters(&mut self, _params: &SamplingParameters) {}
}

impl<T: Copy> SignalGenerator for KnobGenerator<T> {
    type Output = T;

    fn next(&mut self) -> Self::Output {
        self.value.get()
    }
}

impl<T: Copy> Knob<T> {
    pub fn new(initial: T) -> Self {
        Knob {
            value: Rc::new(Cell::new(initial))
        }
    }

    pub fn as_generator(&self) -> KnobGenerator<T> {
        KnobGenerator {
            value: self.value.clone()
        }
    }

    pub fn set(&mut self, value: T) {
        self.value.set(value)
    }

    pub fn get(&self) -> T {
        self.value.get()
    }
}
