use core::hash::{Hash, Hasher};

use ascot::response::ResponseKind;

use serde::Serialize;

use crate::collections::SerialSet;
use crate::hazards::Hazards;
use crate::parameters::{Parameters, ParametersData};

pub use ascot::route::RestKind;

/// Route data.
#[derive(Debug, Clone, Serialize)]
pub struct RouteData<const H: usize, const P: usize> {
    /// Name.
    name: &'static str,
    /// Description.
    description: Option<&'static str>,
    /// Hazards data.
    #[serde(skip_serializing_if = "Hazards::is_empty")]
    hazards: Hazards<H>,
    /// Input parameters associated with a route..
    #[serde(skip_serializing_if = "ParametersData::is_empty")]
    parameters: ParametersData<P>,
}

impl<const H: usize, const P: usize> PartialEq for RouteData<H, P> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<const H: usize, const P: usize> RouteData<H, P> {
    fn new(route: Route<H, P>) -> Self {
        Self {
            name: route.route,
            description: route.description,
            hazards: route.hazards,
            parameters: route.parameters.serialize_data(),
        }
    }
}

/// A server route configuration.
#[derive(Debug, Clone, Serialize)]
pub struct RouteConfig<const H: usize, const P: usize> {
    /// Route.
    #[serde(flatten)]
    data: RouteData<H, P>,
    /// **_REST_** kind..
    #[serde(rename = "REST kind")]
    rest_kind: RestKind,
    /// Response kind.
    #[serde(rename = "response kind")]
    response_kind: ResponseKind,
}

impl<const H: usize, const P: usize> PartialEq for RouteConfig<H, P> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data) && self.rest_kind == other.rest_kind
    }
}

// Hazards and inputs prevent Eq trait to be derived.
impl<const H: usize, const P: usize> Eq for RouteConfig<H, P> {}

impl<const H: usize, const P: usize> Hash for RouteConfig<H, P> {
    fn hash<Ha: Hasher>(&self, state: &mut Ha) {
        self.data.name.hash(state);
        self.rest_kind.hash(state);
    }
}

impl<const H: usize, const P: usize> RouteConfig<H, P> {
    fn new(route: Route<H, P>) -> Self {
        Self {
            rest_kind: route.rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData::new(route),
        }
    }
}

/// A collection of [`RouteConfig`]s.
pub type RouteConfigs<const H: usize, const P: usize, const N: usize> =
    SerialSet<RouteConfig<H, P>, N>;

/// A server route.
///
/// It represents a specific `REST` API which, when invoked, runs a task on
/// a remote device.
#[derive(Debug)]
pub struct Route<const H: usize, const P: usize> {
    // Route.
    route: &'static str,
    // REST kind.
    rest_kind: RestKind,
    // Description.
    description: Option<&'static str>,
    // Hazards.
    hazards: Hazards<H>,
    // Input route parameters.
    parameters: Parameters<P>,
}

impl<const H: usize, const P: usize> PartialEq for Route<H, P> {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.rest_kind == other.rest_kind
    }
}

// Hazards and inputs prevent Eq trait to be derived.
impl<const H: usize, const P: usize> Eq for Route<H, P> {}

impl<const H: usize, const P: usize> Hash for Route<H, P> {
    fn hash<Ha: Hasher>(&self, state: &mut Ha) {
        self.route.hash(state);
    }
}

impl Route<2, 2> {
    /// Creates a new [`Route`] through a REST `GET` API.
    #[must_use]
    pub fn get(route: &'static str) -> Self {
        Self::init(RestKind::Get, route)
    }

    /// Creates a new [`Route`] through a REST `PUT` API.
    #[must_use]
    pub fn put(route: &'static str) -> Self {
        Self::init(RestKind::Put, route)
    }

    /// Creates a new [`Route`] through a REST `POST` API.
    #[must_use]
    pub fn post(route: &'static str) -> Self {
        Self::init(RestKind::Post, route)
    }

    /// Creates a new [`Route`] through a REST `DELETE` API.
    #[must_use]
    pub fn delete(route: &'static str) -> Self {
        Self::init(RestKind::Delete, route)
    }

    fn init(rest_kind: RestKind, route: &'static str) -> Self {
        Route::<2, 2> {
            route,
            rest_kind,
            description: None,
            parameters: Parameters::new(),
            hazards: Hazards::new(),
        }
    }
}

impl<const H: usize, const P: usize> Route<H, P> {
    /// Sets the route description.
    #[must_use]
    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    /// Changes the route.
    #[must_use]
    pub const fn change_route(mut self, route: &'static str) -> Self {
        self.route = route;
        self
    }

    /// Adds [`Hazards`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_hazards<const H2: usize>(self, hazards: Hazards<H2>) -> Route<H2, P> {
        Route::<H2, P> {
            route: self.route,
            rest_kind: self.rest_kind,
            description: self.description,
            parameters: self.parameters,
            hazards,
        }
    }

    /// Adds [`Parameters`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_parameters<const P2: usize>(self, parameters: Parameters<P2>) -> Route<H, P2> {
        Route::<H, P2> {
            route: self.route,
            rest_kind: self.rest_kind,
            description: self.description,
            parameters,
            hazards: self.hazards,
        }
    }

    /// Returns route.
    #[must_use]
    pub const fn route(&self) -> &str {
        self.route
    }

    /// Returns [`RestKind`].
    #[must_use]
    pub const fn kind(&self) -> RestKind {
        self.rest_kind
    }

    /// Returns [`Hazards`].
    #[must_use]
    pub const fn hazards(&self) -> &Hazards<H> {
        &self.hazards
    }

    /// Returns [`Parameters`].
    #[must_use]
    pub const fn parameters(&self) -> &Parameters<P> {
        &self.parameters
    }

    /// Serializes [`Route`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> RouteConfig<H, P> {
        RouteConfig::new(self)
    }
}

#[cfg(test)]
mod tests {
    use ascot::hazards::Hazard;
    use serde_json::json;

    use crate::hazards::Hazards;
    use crate::parameters::{ParameterKind, Parameters};
    use crate::serialize;

    use super::Route;

    #[test]
    fn test_all_routes() {
        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A GET route",
                "REST kind": "Get",
                "response kind": "Ok"
            })
        );

        assert_eq!(
            serialize(
                Route::put("/route")
                    .description("A PUT route")
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A PUT route",
                "REST kind": "Put",
                "response kind": "Ok"
            })
        );

        assert_eq!(
            serialize(
                Route::post("/route")
                    .description("A POST route")
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A POST route",
                "REST kind": "Post",
                "response kind": "Ok"
            })
        );

        assert_eq!(
            serialize(
                Route::delete("/route")
                    .description("A DELETE route")
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A DELETE route",
                "REST kind": "Delete",
                "response kind": "Ok"
            })
        );
    }

    #[test]
    fn test_all_hazards() {
        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazards(Hazards::three((
                        Hazard::FireHazard,
                        Hazard::AirPoisoning,
                        Hazard::Explosion
                    )))
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A GET route",
                "REST kind": "Get",
                "response kind": "Ok",
                "hazards": [
                    "FireHazard",
                    "AirPoisoning",
                    "Explosion",
                ],
            })
        );
    }

    #[test]
    fn test_all_parameters() {
        let expected = json!({
            "name": "/route",
            "description": "A GET route",
            "REST kind": "Get",
            "response kind": "Ok",
            "parameters": {
                    "rangeu64": {
                        "RangeU64": {
                            "min": 0,
                            "max": 20,
                            "step": 1,
                            "default": 5
                        }
                    },
                    "rangef64": {
                        "RangeF64": {
                            "min": 0.0,
                            "max": 20.0,
                            "step": 0.1,
                            "default": 0.0
                        }
                    }
             },
            "REST kind": "Get"
        });

        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_parameters(Parameters::two((
                        (
                            "rangeu64",
                            ParameterKind::rangeu64_with_default((0, 20, 1), 5)
                        ),
                        ("rangef64", ParameterKind::rangef64((0., 20., 0.1)))
                    )))
                    .serialize_data()
            ),
            expected
        );
    }

    #[test]
    fn test_complete_route() {
        let expected = json!({
            "name": "/route",
            "description": "A GET route",
            "REST kind": "Get",
            "response kind": "Ok",
            "hazards": [
                    "FireHazard",
                    "AirPoisoning",
                    "Explosion",
            ],
            "parameters": {
                    "rangeu64": {
                        "RangeU64": {
                            "min": 0,
                            "max": 20,
                            "step": 1,
                            "default": 5
                        }
                    },
                    "rangef64": {
                        "RangeF64": {
                            "min": 0.0,
                            "max": 20.0,
                            "step": 0.1,
                            "default": 0.0
                        }
                    }
             },
            "REST kind": "Get"
        });

        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazards(Hazards::three((
                        Hazard::FireHazard,
                        Hazard::AirPoisoning,
                        Hazard::Explosion
                    )))
                    .with_parameters(Parameters::two((
                        (
                            "rangeu64",
                            ParameterKind::rangeu64_with_default((0, 20, 1), 5)
                        ),
                        ("rangef64", ParameterKind::rangef64((0., 20., 0.1)))
                    )))
                    .serialize_data()
            ),
            expected
        );
    }
}
