use crate::collections::create_set;

pub use ascot::hazards::{Category, Hazard, HazardData, ALL_HAZARDS};

create_set!(Hazards, Hazard, hazard, hazards);
