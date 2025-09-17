use alloc::vec::Vec;

use ascot::device::DeviceKind;
use ascot::route::Route;

use picoserve::routing::{NoPathParameters, PathRouter, Router};

/// A generic device.
pub struct Device<PR: PathRouter<(), NoPathParameters>> {
    pub(crate) main_route: &'static str,
    pub(crate) kind: DeviceKind,
    pub(crate) routes: Vec<Route>,
    pub(crate) num_mandatory_routes: u8,
    pub(crate) internal_router: Router<PR, (), NoPathParameters>,
}

impl<PR: PathRouter<(), NoPathParameters>> Device<PR> {
    pub(crate) const fn new(
        main_route: &'static str,
        kind: DeviceKind,
        routes: Vec<Route>,
        num_mandatory_routes: u8,
        internal_router: Router<PR, (), NoPathParameters>,
    ) -> Self {
        Self {
            main_route,
            kind,
            routes,
            num_mandatory_routes,
            internal_router,
        }
    }
}
