use alloc::vec::Vec;

use ascot::device::{DeviceData, DeviceEnvironment, DeviceKind};
use ascot::route::{Route, RouteConfigs};

use picoserve::response::json::Json;
use picoserve::routing::{
    get, MethodHandler, NoPathParameters, PathDescription, PathRouter, Router,
};

use crate::mk_static;

/// A generic device.
pub struct Device<PR: PathRouter<(), NoPathParameters>> {
    main_route: &'static str,
    kind: DeviceKind,
    routes: Vec<Route>,
    num_mandatory_routes: u8,
    pub(crate) router: Router<PR>,
}

impl<PR: PathRouter<(), NoPathParameters>> Device<PR> {
    pub(crate) fn new(
        main_route: &'static str,
        kind: DeviceKind,
        routes: Vec<Route>,
        num_mandatory_routes: u8,
        router: Router<PR>,
    ) -> Self {
        Self {
            main_route,
            kind,
            routes,
            num_mandatory_routes,
            router,
        }
    }

    pub(crate) fn finalize(self) -> Router<impl PathRouter> {
        let router = self.router;

        let mut route_configs = RouteConfigs::new();
        for route in self.routes {
            route_configs.add(route.serialize_data());
        }

        let device_data = DeviceData::new(
            self.kind,
            DeviceEnvironment::Esp32,
            self.main_route,
            route_configs,
            None,
            None,
            self.num_mandatory_routes,
        );

        let response = &*mk_static!(DeviceData, device_data);

        router.route("/", get(move || async move { Json(response) }))
    }
}
