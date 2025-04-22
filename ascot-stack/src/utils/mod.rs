// All sets collections needed for internal storage and I/O tasks.
mod sets;
// All maps collections needed for internal storage and I/O tasks.
mod map;

/// A fixed-capacity string.
pub mod string;

/// All supported collections.
pub mod collections {
    pub(crate) use super::map::create_map;
    pub(crate) use super::sets::create_set;
    pub use super::sets::{OutputSet, SerialSet, Set};
}
