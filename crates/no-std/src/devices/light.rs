use alloc::vec::Vec;

use ascot::device::DeviceKind;
use ascot::hazards::Hazard;
use ascot::route::Route;

use picoserve::routing::{
    MethodHandler, NoPathParameters, NotFound, PathDescription, PathRouter, Router,
};

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
/// This structure is just a placeholder to guide the construction
/// of a [`CompleteLight`].
pub struct Light;

impl Light {
    /// Creates a [`LightOnRoute`] containing the route to turn a light on only.
    ///
    /// This method represents the first step to build a [`CompleteLight`].
    pub fn turn_light_on(
        route: ascot::route::LightOnRoute,
        handler: impl MethodHandler<(), <&'static str as PathDescription<NoPathParameters>>::Output>,
    ) -> LightOnRoute<impl PathRouter<(), NoPathParameters>> {
        let route = route.into_route();
        let router = Router::new();
        let mut routes = Vec::new();

        let router = route_checks(route, &mut routes, router, handler);

        LightOnRoute(CompleteLight {
            main_route: LIGHT_MAIN_ROUTE,
            routes,
            router,
        })
    }
}

/// A `light` containing only the route to turn a light on.
///
/// You must call its only method to build a [`CompleteLight`].
pub struct LightOnRoute<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = NoPathParameters,
>(CompleteLight<PR, State, CurrentPathParameters>);

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    LightOnRoute<PR, State, CurrentPathParameters>
{
    /// Creates a [`CompleteLight`].
    pub fn turn_light_off(
        mut self,
        route: ascot::route::LightOffRoute,
        handler: impl MethodHandler<
            State,
            <&'static str as PathDescription<CurrentPathParameters>>::Output,
        >,
    ) -> CompleteLight<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    {
        let route = route.into_route();

        let router = route_checks(route, &mut self.0.routes, self.0.router, handler);

        CompleteLight {
            main_route: self.0.main_route,
            routes: self.0.routes,
            router,
        }
    }
}

/// A complete `light` device with all the mandatory routes set.
pub struct CompleteLight<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = NoPathParameters,
> {
    main_route: &'static str,
    routes: Vec<Route>,
    router: Router<PR, State, CurrentPathParameters>,
}

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    CompleteLight<PR, State, CurrentPathParameters>
{
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
        handler: impl MethodHandler<
            State,
            <&'static str as PathDescription<CurrentPathParameters>>::Output,
        >,
    ) -> CompleteLight<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    {
        let router = route_checks(route, &mut self.routes, self.router, handler);

        CompleteLight {
            main_route: self.main_route,
            routes: self.routes,
            router,
        }
    }

    /// Builds a [`Device`].
    pub fn build(self) -> Device<PR, State, CurrentPathParameters> {
        Device::new(self.router)
    }
}
