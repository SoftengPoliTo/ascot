#![no_std]
// #![feature(impl_trait_in_assoc_type)]

extern crate alloc;

pub mod devices;

pub mod device;
pub mod error;
pub mod wifi;
pub mod net;
pub mod mdns;
pub mod server;
pub mod mqtt;

pub use picoserve::routing::{get, post, put, delete};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub(crate) use mk_static;