//! The communication interface among an Ascot device and an Ascot controller
//! for stack-oriented Ascot devices.
//!
//! This interface must be used **only** by stack-oriented Ascot devices, and
//! not by Ascot controllers, since **not** all structures can be deserialized.
//! This choice has been made to avoid allocations, reduce the crate size and
//! the number of operations performed.
//! However, a correct structures serialization process is **always**
//! guaranteed.
//!
//! For heap-oriented Ascot devices, the main
//! [ascot](https://github.com/SoftengPoliTo/ascot/tree/master/src)
//! crate is more suitable.
//!
//! This crate contains a series of APIs to:
//!
//! - Encode and decode the information about a device structure and
//!   all of its routes. A route is an address which a controller can invoke
//!   to execute one or more device operations.
//! - Manage the hazards which might occur when the operations invoked by a
//!   route are executed. Hazards describe all safety, privacy, and financial
//!   problems associated with a route invocation. They can also be employed
//!   to manage the events occurring on a device.
//! - Manage the possible input parameters of a route. An input parameter
//!   might represent an external information needed to perform a device
//!   operation or a condition to block or allow determined instructions.
//!   For example, a boolean parameter might delineate the on/off states of a
//!   light, but also a condition to discriminate among these two states.
//!   Instead, a range-of-floats parameter might be adopted to control the
//!   light brightness state.
//!
//! To share data among a device and a controller, this interface provides the
//! same `ascot` crate structures. A stack-oriented device fills in
//! these structures with the desired data, while a controller consumes their
//! content to retrieve the device information.
//!
//! This crate can be used **ONLY** on `no_std` environments. If a `std`
//! environment is requested, better to use the main
//! [ascot](https://github.com/SoftengPoliTo/ascot/tree/master/src)
//! crate.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

/// All methods to interact with an action.
pub mod actions {
    pub use ascot::actions::ActionError;
}
/// Description of a device with its routes information.
pub mod device;
/// Information about the economy device aspects.
pub mod economy;
/// Information about the energy device aspects.
pub mod energy;
/// Error handling.
pub mod error;
/// Hazards descriptions and methods.
pub mod hazards;
/// Route input parameters.
pub mod parameters;
/// All supported responses returned by a device action.
pub mod response;
/// Definition of device routes.
pub mod route;

// All fixed-capacity structures and collections.
mod utils;
pub use utils::{collections, string};

#[cfg(test)]
pub(crate) fn serialize<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}
