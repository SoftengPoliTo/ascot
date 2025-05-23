use crate::collections::OutputSet;

pub use ascot::hazards::{ALL_HAZARDS, Category, Hazard, HazardData};

/// A collection of [`Hazard`]s.
///
/// **For alignment reasons, it accepts only a power of two
/// as number of elements.**
pub type Hazards<const N: usize> = OutputSet<Hazard, N>;
