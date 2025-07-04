//! A crate library for building Ascot devices firmware running on
//! operating systems.
//!
//! This crate has been conceived for those devices which require a great amount
//! of resources for their execution, in terms of computing times, memory size,
//! and connected components.
//!
//! As of now, only devices for `x86_64` and `ARM` hardware architectures have
//! been tested.
//!
//! All devices firmware are modelled on real use cases. Each device is
//! composed of a description and a set of operations with their own
//! hazards associated.
//!
//! A description is defined as a sequence of fields, such as
//! the device name, kind, and other information to set a secure
//! communication channel. However, the sets of actions are the ones which
//! better characterize a device.
//!
//! An **action** represents a sequence of one or more operations executed
//! on a device when an external device invokes a determined server route.
//! Each action is **always** coupled with a route which can have zero or
//! more hazards.
//!
//! If an action has no hazards, that action might arise unknown dangers. It
//! will be a controller responsibility to evaluate if the invoked action must
//! be blocked or not.
//!
//! This crate cannot check at compile time all possible effects which
//! might occur in a determined environment while an action is executed.
//! Indeed, hazards represent only informative data usable by a controller,
//! to block or allow the invocation of an action according to
//! some privacy policies.
//!
//! An `std` environment is mandatory to make a full usage of the provided
//! functionalities.

#![deny(unsafe_code)]
#![deny(missing_docs)]

/// All device kinds implementable in a firmware.
pub mod devices;

/// All action kinds with their payloads.
pub mod actions;
/// Methods to define a device and its actions.
pub mod device;
/// Error handling.
pub mod error;
/// Methods to define and run the server which represents the firwmare.
pub mod server;
/// Methods to define and run the discovery service necessary to detect a
/// device in a network.
pub mod service;

/// Methods to parse requests and construct responses.
pub mod extract {
    pub use axum::extract::{FromRef, Json, Path, State};
    pub use axum::http::header;
}

mod services;

mod mac;
