use ascot::hazards::Hazard;

use crate::collections::OutputCollection;

/// A collection of [`Hazard`]s.
pub type Hazards<const N: usize> = OutputCollection<Hazard<N>>;
