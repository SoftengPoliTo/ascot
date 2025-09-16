use alloc::vec::Vec;

use ascot::device::DeviceKind;
use ascot::hazards::Hazard;
use ascot::route::Route;

use esp_wifi::wifi::WifiDevice;

use picoserve::routing::{MethodHandler, NoPathParameters, PathDescription, PathRouter, Router};

use crate::device::Device;

// The default main route for a light.
const LIGHT_MAIN_ROUTE: &str = "/light";

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

#[inline]
fn route_checks<
    PR: PathRouter<State, CurrentPathParameters>,
    State,
    CurrentPathParameters,
    T: MethodHandler<State, <&'static str as PathDescription<CurrentPathParameters>>::Output>,
>(
    route: Route,
    routes: &mut Vec<Route>,
    router: Router<PR, State, CurrentPathParameters>,
    handler: T,
) -> Router<
    impl PathRouter<State, CurrentPathParameters> + use<PR, State, CurrentPathParameters, T>,
    State,
    CurrentPathParameters,
> {
    // TODO: Check hazards
    // Return an error if action hazards are not a subset of allowed hazards.
    /*for hazard in route.hazards() {
        if !ALLOWED_HAZARDS.contains(hazard) {
            return Err(Error::new(ErrorKind::Device, "Hazard not allowed"));
        }
    }*/

    let router = router.route(route.static_route(), handler);

    routes.push(route);

    router
}

/// A `light` device.
///
/// This structure serves as the initial placeholder for constructing
/// a [`CompleteLight`].
pub struct Light([u8; 6]);

impl Light {
    /// Creates a [`Light`].
    pub fn new(wifi_interface: &WifiDevice<'_>) -> Self {
        Self(wifi_interface.mac_address())
    }

    /// Creates a [`LightOnRoute`] that exclusively includes the route for
    /// turning a light on.
    ///
    /// This method **must** be called **first** to initialize and construct
    /// a [`CompleteLight`].
    pub fn turn_light_on(
        self,
        route: ascot::route::LightOnRoute,
        handler: impl MethodHandler<(), <&'static str as PathDescription<NoPathParameters>>::Output>,
    ) -> LightOnRoute<impl PathRouter<(), NoPathParameters>> {
        let route = route.into_route();
        let router = Router::new();
        let mut routes = Vec::new();

        let router = route_checks(route, &mut routes, router, handler);

        LightOnRoute(CompleteLight {
            id: self.0,
            main_route: LIGHT_MAIN_ROUTE,
            routes,
            router,
        })
    }
}

/// A `light` device configured with only the route to turn the light on.
///
/// You need to invoke its sole method to construct a [`CompleteLight`].
pub struct LightOnRoute<PR: PathRouter<(), NoPathParameters>>(CompleteLight<PR>);

impl<PR: PathRouter<(), NoPathParameters>> LightOnRoute<PR> {
    /// Creates a [`CompleteLight`].
    ///
    /// This method **must** be called **second** to initialize and construct
    /// a [`CompleteLight`].
    pub fn turn_light_off(
        mut self,
        route: ascot::route::LightOffRoute,
        handler: impl MethodHandler<(), <&'static str as PathDescription<NoPathParameters>>::Output>,
    ) -> CompleteLight<impl PathRouter<(), NoPathParameters>> {
        let route = route.into_route();

        let router = route_checks(route, &mut self.0.routes, self.0.router, handler);

        CompleteLight {
            id: self.0.id,
            main_route: self.0.main_route,
            routes: self.0.routes,
            router,
        }
    }
}

/// A fully configured `light` device with all mandatory routes initialized.
pub struct CompleteLight<PR: PathRouter<(), NoPathParameters>> {
    id: [u8; 6],
    main_route: &'static str,
    routes: Vec<Route>,
    router: Router<PR>,
}

impl<PR: PathRouter<(), NoPathParameters>> CompleteLight<PR> {
    /// Sets a new main route.
    #[must_use]
    pub const fn main_route(mut self, main_route: &'static str) -> Self {
        self.main_route = main_route;
        self
    }

    /// Adds an additional route to a [`CompleteLight`].
    pub fn route(
        mut self,
        route: Route,
        handler: impl MethodHandler<(), <&'static str as PathDescription<NoPathParameters>>::Output>,
    ) -> CompleteLight<impl PathRouter<(), NoPathParameters>> {
        let router = route_checks(route, &mut self.routes, self.router, handler);

        CompleteLight {
            id: self.id,
            main_route: self.main_route,
            routes: self.routes,
            router,
        }
    }

    /// Builds a [`Device`].
    pub fn build(self) -> Device<PR> {
        Device::new(
            self.main_route,
            DeviceKind::Light,
            self.routes,
            2,
            self.router,
        )
    }
}
