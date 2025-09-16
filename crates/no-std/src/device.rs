use alloc::boxed::Box;
use alloc::vec::Vec;

use ascot::device::{DeviceData, DeviceEnvironment, DeviceKind};
use ascot::route::{Route, RouteConfigs};

use picoserve::response::{json::Json, IntoResponse};
use picoserve::routing::{get, NoPathParameters, PathRouter, Router};

use crate::mk_static;

/// A generic device.
pub struct Device<
    PR: PathRouter<(), CurrentPathParameters>,
    CurrentPathParameters = NoPathParameters,
> {
    main_route: &'static str,
    kind: DeviceKind,
    routes: Vec<Route>,
    num_mandatory_routes: u8,
    internal_router: Router<PR, (), CurrentPathParameters>,
}

impl<PR: PathRouter<(), CurrentPathParameters>, CurrentPathParameters>
    Device<PR, CurrentPathParameters>
{
    pub(crate) fn new(
        main_route: &'static str,
        kind: DeviceKind,
        routes: Vec<Route>,
        num_mandatory_routes: u8,
        internal_router: Router<PR, (), CurrentPathParameters>,
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

async fn main_route(device_data: DeviceData) -> Json<DeviceData> {
    Json(device_data)
}

impl<PR: PathRouter<(), NoPathParameters>> Device<PR> {
    pub(crate) fn finalize(self) -> Router<PR> {
        let router = self.internal_router;

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

        //let response = &*mk_static!(DeviceData, device_data);

        let router = Router::new().route("/", get(|| async move { Json(device_data) }));

        //let router = add_route("/", router, get(|| main_route(device_data)));

        router
    }
}

use picoserve::routing::{MethodHandler, PathDescription};

#[inline]
fn add_route<
    PR: PathRouter<(), NoPathParameters>,
    T: MethodHandler<(), <&'static str as PathDescription<NoPathParameters>>::Output>,
>(
    route: &'static str,
    internal_router: Router<PR, (), NoPathParameters>,
    handler: T,
) -> Router<impl PathRouter<(), NoPathParameters>, (), NoPathParameters> {
    internal_router.route(route, handler)
}
