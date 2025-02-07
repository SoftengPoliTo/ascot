//! The communication interface between a stack-oriented device and a more
//! general-purpose device which acts as controller.
//!
//! This interface is thought to be used **only** on the stack-oriented device
//! side because not all structures cannot be deserialized.
//! However, it guarantees a correct data serialization for information sent to
//! a controller.
//!
//! For a heap-oriented device, the main `ascot` crate is more suitable.
//!
//! This crate contains a series of APIs to:
//!
//! - Encode and decode the description file containing a device structure and
//!   all of its routes. A route is expressed as an address which can be invoked
//!   by a controller to execute an action on a device.
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
pub mod input;
/// All supported responses returned by a device action.
pub mod response;
/// Definition of device routes.
pub mod route;
/// All fixed-capacity structures and collections.
pub mod utils;

#[cfg(test)]
pub(crate) fn serialize<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

#[cfg(test)]
pub(crate) fn deserialize<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).unwrap()
}
