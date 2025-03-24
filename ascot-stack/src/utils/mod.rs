// All maps collections needed for internal storage and I/O tasks.
mod maps;
// All sets collections needed for internal storage and I/O tasks.
mod sets;

/// A fixed-capacity string.
pub mod string;

/// All supported collections.
pub mod collections {
    pub use super::maps::{Map, OutputMap, SerialMap};
    pub(crate) use super::sets::create_set;
    pub use super::sets::{OutputSet, SerialSet, Set};
}
