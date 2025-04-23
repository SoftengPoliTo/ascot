// An internal set implmentation.
mod set;
// An internal map implementation.
mod map;

/// A fixed-capacity string.
pub mod string;

/// All supported collections.
pub mod collections {
    pub(crate) use super::map::create_map;
    pub(crate) use super::set::create_set;
    pub use super::set::{OutputSet, SerialSet, Set};
}
