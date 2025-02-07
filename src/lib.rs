//! The communication interface among a device and a controller.
//!
//! This crate contains a series of APIs to:
//!
//! - Encode and decode the file containing the description of a device and
//!   all of its routes. A route is expressed as an address which a controller
//!   can invoke to execute an action on a device.
//! - Manage the hazards which might occur on a device when a determined route
//!   is being invoked. Hazards can also be employed to manage the events
//!   happening on a device.
//! - Manage the input parameters of a route. An input parameter represents
//!   an argument for a device action. For example, a boolean which
//!   controls the state of a light, or a range of floats to control the
//!   brightness of a light.
//!
//! It also provides some structures to share data among a device and
//! a controller. Each of these structures must be both serializable and
//! deserializable. A device fills in these structures, while a controller
//! consumes them.
//!
//! This crate can be used both on `std` and `no_std` environments. Indeed, the
//! `std` features is enabled by default.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All methods to interact with an action.
pub mod actions;
/// Description of a device with its routes information.
pub mod device;
/// Information about the economy device aspects.
pub mod economy;
/// Information about the energy device aspects.
pub mod energy;
/// Hazards descriptions and methods.
pub mod hazards;
/// Route input parameters.
#[cfg(feature = "alloc")]
pub mod input;
/// All supported responses returned by a device action.
pub mod response;
/// Definition of device routes.
pub mod route;
/// All structures and collections.
#[cfg(feature = "alloc")]
pub mod utils;

#[cfg(test)]
pub(crate) fn serialize<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

#[cfg(test)]
pub(crate) fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).unwrap()
}
