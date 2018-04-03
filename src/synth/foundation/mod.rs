#[macro_use]
pub mod module;
pub mod types;
pub mod sample;
pub mod generator;
pub mod filter;

pub use sample::{Sample, Resample};
pub use module::{SoundModule, SamplingParameters, Parameter};
pub use generator::{SignalGenerator};
pub use filter::Filter;
pub use types::{Frequency, Duration, units};
