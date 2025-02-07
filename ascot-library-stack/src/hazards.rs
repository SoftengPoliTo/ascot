use ascot_library::hazards::Hazard;

use crate::utils::collections::OutputCollection;

/// A collection of [`Hazard`]s.
pub type Hazards<const N: usize> = OutputCollection<Hazard, N>;
