#[macro_use]
pub mod module;
pub mod types;
pub mod sample;
pub mod generator;
pub mod filter;

pub use self::sample::{Sample, Resample};
pub use self::module::{SoundModule, SamplingParameters};
pub use self::generator::{SignalGenerator};
pub use self::filter::Filter;
pub use self::types::{Frequency, Duration, units};
