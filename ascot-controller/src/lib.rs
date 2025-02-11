//! A crate library to manage, orchestrate, and interact with all
//! `ascot`-compliant devices present in a network.
//!
//! Among its tasks:
//!
//! - Discovering all `ascot-compliant` devices contained in a network
//! - Building `REST` requests to send commands to the discovered devices
//! - Defining scheduling programs to control requests sending
//! - Setting security and privacy policies to allow or prevent a request
//!   from being sent
//!
//! The possibility of defining scheduling programs allows to implement
//! batch processing, hence all those requests which have determined properties
//! can be grouped together and sent to different devices either immediately or
//! at a later time.
//!
//! Some APIs invoke threads to perform their operations. This choice has been
//! taken to split up huge tasks into small independent ones and distribute the
//! system resource load.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// A controller to manage how requests are sent to a device.
pub mod controller;
/// A compliant device.
pub mod device;
/// A discovery mechanism to identify all compliant devices in a network.
pub mod discovery;
/// Error handling.
pub mod error;
/// All supported request input parameters.
pub mod parameters;
/// A privacy and security policy manager to allow or prevent a request
/// from being sent.
pub mod policy;
/// All requests data and methods.
pub mod request;
/// All supported device responses methods and data.
pub mod response;
/// A scheduler to define tasks related to a request sending.
pub mod scheduler;

#[cfg(test)]
mod tests;
