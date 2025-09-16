#![no_std]

extern crate alloc;

pub mod devices;

pub mod device;
pub mod error;
pub mod mdns;
pub mod mqtt;
pub mod net;
pub mod server;
pub mod wifi;

pub use picoserve::routing::{delete, get, parse_path_segment, post, put};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub(crate) use mk_static;
