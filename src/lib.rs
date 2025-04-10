//! The communication interface among an Ascot device and an Ascot controller.
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
//! To share data among a device and a controller, each structure of this
//! interface must be both serializable and deserializable.
//! A device fills in these structures with the desired data, while a controller
//! consumes their content to retrieve the device information.
//!
//! This crate can be used both on `std` and `no_std` environments. The `alloc`
//! feature allows heap-allocations and it is enabled by default.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All methods to interact with an action.
pub mod actions;
/// All data collections.
#[cfg(feature = "alloc")]
pub mod collections;
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
pub mod parameters;
/// All supported responses returned by a device action.
pub mod response;
/// Definition of device routes.
pub mod route;

#[cfg(test)]
pub(crate) fn serialize<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

#[cfg(feature = "deserialize")]
#[cfg(test)]
pub(crate) fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).unwrap()
}
