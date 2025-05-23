/// Action and response to manage errors.
pub mod error;
/// Action and response to manage device information.
pub mod info;
/// Action and response to confirm the correct execution of an action.
pub mod ok;
/// Action and response to manage data serialization.
pub mod serial;
/// Action and response to manage a stream of data expressed as a sequence
/// of bytes.
pub mod stream;

use ascot::hazards::{Hazard, Hazards};
use ascot::parameters::{ParameterKind, ParametersData};
use ascot::response::ResponseKind;
use ascot::route::{RestKind, Route, RouteConfig};

use axum::{Router, handler::Handler};

use tracing::{error, info, warn};

use std::fmt::Write;

#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([], );
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}

pub(super) use all_the_tuples;

fn build_get_route(route: &str, parameters: &ParametersData) -> String {
    let mut route = String::from(route);
    for (name, parameter_kind) in parameters {
        if matches!(parameter_kind, ParameterKind::ByteStream) {
            warn!("A bytes stream is not accepted for `GET` requests, skip it");
            continue;
        }
        // TODO: Consider returning `Option<String>`
        if let Err(e) = write!(route, "/{{{name}}}") {
            error!("Error in adding a path to a route : {e}");
            break;
        }
    }
    info!("Build GET route: {}", route);
    route
}

#[derive(Debug)]
/// A generic [`crate::device::Device`] action.
///
/// It has been conceived to perform checks on [`Hazard`]s.
pub struct DeviceAction {
    // Router.
    pub(crate) router: Router,
    // Route
    pub(crate) route_config: RouteConfig,
}

impl DeviceAction {
    /// Checks whether an action does not define the given [`Hazard`].
    #[must_use]
    #[inline]
    pub fn miss_hazard(&self, hazard: Hazard) -> bool {
        !self.route_config.data.hazards.contains(&hazard)
    }

    /// Checks whether an action does not define the given [`Hazard`]s.
    #[must_use]
    #[inline]
    pub fn miss_hazards(&self, hazards: &'static [Hazard]) -> bool {
        !hazards
            .iter()
            .all(|hazard| self.route_config.data.hazards.contains(hazard))
    }

    /// Returns the [`Hazards`] collection associated with an action.
    #[must_use]
    #[inline]
    pub fn hazards(&self) -> &Hazards {
        &self.route_config.data.hazards
    }

    pub(crate) fn stateless<H, T>(route: Route, response_kind: ResponseKind, handler: H) -> Self
    where
        H: Handler<T, ()>,
        T: 'static,
    {
        Self::init(route, response_kind, handler, ())
    }

    pub(crate) fn stateful<H, T, S>(
        route: Route,
        response_kind: ResponseKind,
        handler: H,
        state: S,
    ) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
        Self::init(route, response_kind, handler, state)
    }

    fn init<H, T, S>(route: Route, response_kind: ResponseKind, handler: H, state: S) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
        S: Clone + Send + Sync + 'static,
    {
        let mut route_config = route.serialize_data();
        route_config.response_kind = response_kind;

        // Create the GET route for the axum architecture.
        let route = if matches!(route_config.rest_kind, RestKind::Get)
            && !route_config.data.parameters.is_empty()
        {
            &build_get_route(&route_config.data.name, &route_config.data.parameters)
        } else {
            route_config.data.name.as_ref()
        };

        let router = Router::new()
            .route(
                route,
                match route_config.rest_kind {
                    RestKind::Get => axum::routing::get(handler),
                    RestKind::Put => axum::routing::put(handler),
                    RestKind::Post => axum::routing::post(handler),
                    RestKind::Delete => axum::routing::delete(handler),
                },
            )
            .with_state(state);

        Self {
            router,
            route_config,
        }
    }
}

/// A mandatory [`DeviceAction`].
pub struct MandatoryAction<const SET: bool> {
    pub(crate) device_action: DeviceAction,
}

impl MandatoryAction<false> {
    pub(crate) fn empty() -> Self {
        Self {
            device_action: DeviceAction {
                router: Router::new(),
                route_config: Route::get("").serialize_data(),
            },
        }
    }

    pub(super) const fn new(device_action: DeviceAction) -> Self {
        Self { device_action }
    }
}

impl MandatoryAction<true> {
    /// Returns a [`DeviceAction`] reference.
    #[must_use]
    pub const fn action_as_ref(&self) -> &DeviceAction {
        &self.device_action
    }

    pub(crate) const fn init(device_action: DeviceAction) -> Self {
        Self { device_action }
    }
}

#[cfg(test)]
mod tests {
    use ascot::parameters::Parameters;

    use super::{Route, build_get_route};

    #[test]
    fn test_build_get_route() {
        let route = Route::get("/route")
            .description("A GET route.")
            .with_parameters(
                Parameters::new()
                    .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
                    .rangef64("rangef64", (0., 20., 0.1)),
            )
            .serialize_data();

        assert_eq!(
            &build_get_route(&route.data.name, &route.data.parameters),
            "/route/{rangeu64}/{rangef64}"
        );
    }
}
