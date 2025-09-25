//! A Rust library crate designed to develop `Ascot` firmware on `ESP32-C3`
//! boards.
//!
//! It offers APIs to:
//!
//! - Connect a device to a `Wi-Fi` access point
//! - Build the network stack
//! - Configure the `mDNS-SD` discovery service
//! - Initialize and run an `HTTP` server
//!
//! The device APIs have been conceived to assist developers in defining their
//! own devices, minimizing as much as possible the ambiguities that may arise
//! during firmware development.
//!
//! Some of the most common errors include:
//!
//! - Absence of the fundamental methods that define a device
//! - Missing or incorrect hazard information associated with a server route
//!
//! To ensure device customization, there are also APIs to add routes
//! associated with specific device operations.
//!
//! Each device route should be associated with one or more hazards, serving
//! as informative indicators of the potential risks involved in invoking an
//! operation.
//! It is then the controller's responsibility to evaluate these hazards and
//! decide whether to block or allow the invocation of the operation.
//!
//! This crate cannot verify at compile time all potential effects that
//! may occur while an operation is running.
//! Actually, hazards are purely informational data for a controller, that
//! determines whether to block or permit the invocation of an operation
//! based on established privacy policies.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

/// All device types implementable within firmware.
pub mod devices;

/// A general and immutable device.
pub mod device;
/// The error manager.
pub mod error;
/// All methods to configure the `mDNS-SD` service.
pub mod mdns;
/// All methods associated with the network stack.
pub mod net;
/// All supported response types.
pub mod response;
/// All methods to initialize and run the firmware server.
pub mod server;
/// A device state.
pub mod state;
/// All methods to configure and connect to a `Wi-Fi` access point.
pub mod wifi;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub(crate) use mk_static;
