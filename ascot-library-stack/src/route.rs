use core::hash::{Hash, Hasher};

use ascot_library::response::ResponseKind;
use ascot_library::route::RestKind;

use serde::Serialize;

use crate::hazards::Hazards;
use crate::input::{Inputs, InputsData};
use crate::utils::collections::{Collection, SerialCollection};

/// Route data.
#[derive(Debug, Clone, Serialize)]
pub struct RouteData<const H: usize, const I: usize> {
    /// Name.
    name: &'static str,
    /// Description.
    description: Option<&'static str>,
    /// Hazards data.
    #[serde(skip_serializing_if = "Hazards::is_empty")]
    #[serde(default = "Hazards::empty")]
    hazards: Hazards<H>,
    /// Inputs associated with a route..
    #[serde(skip_serializing_if = "InputsData::is_empty")]
    #[serde(default = "InputsData::empty")]
    inputs: InputsData<I>,
}

impl<const H: usize, const I: usize> PartialEq for RouteData<H, I> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<const H: usize, const I: usize> RouteData<H, I> {
    fn new(route: Route<H, I>) -> Self {
        Self {
            name: route.route,
            description: route.description,
            hazards: route.hazards,
            inputs: InputsData::from(route.inputs),
        }
    }
}

/// A server route configuration.
#[derive(Debug, Clone, Serialize)]
pub struct RouteConfig<const H: usize, const I: usize> {
    /// Route.
    #[serde(flatten)]
    data: RouteData<H, I>,
    /// **_REST_** kind..
    #[serde(rename = "REST kind")]
    rest_kind: RestKind,
    /// Response kind.
    #[serde(rename = "response kind")]
    response_kind: ResponseKind,
}

impl<const H: usize, const I: usize> PartialEq for RouteConfig<H, I> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data) && self.rest_kind == other.rest_kind
    }
}

// Hazards and inputs prevent Eq trait to be derived.
impl<const H: usize, const I: usize> Eq for RouteConfig<H, I> {}

impl<const H: usize, const I: usize> Hash for RouteConfig<H, I> {
    fn hash<Ha: Hasher>(&self, state: &mut Ha) {
        self.data.name.hash(state);
        self.rest_kind.hash(state);
    }
}

impl<const H: usize, const I: usize> RouteConfig<H, I> {
    fn new(route: Route<H, I>) -> Self {
        Self {
            rest_kind: route.rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData::new(route),
        }
    }
}

/// A collection of [`RouteConfig`]s.
pub type RouteConfigs<const H: usize, const I: usize, const N: usize> =
    SerialCollection<RouteConfig<H, I>, N>;

/// A server route.
///
/// It represents a specific `REST` API which, when invoked, runs a task on
/// a remote device.
#[derive(Debug)]
pub struct Route<const H: usize, const I: usize> {
    // Route.
    route: &'static str,
    // REST kind.
    rest_kind: RestKind,
    // Description.
    description: Option<&'static str>,
    // Hazards.
    hazards: Hazards<H>,
    // Inputs.
    inputs: Inputs<I>,
}

impl<const H: usize, const I: usize> PartialEq for Route<H, I> {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.rest_kind == other.rest_kind
    }
}

// Hazards and inputs prevent Eq trait to be derived.
impl<const H: usize, const I: usize> Eq for Route<H, I> {}

impl<const H: usize, const I: usize> Hash for Route<H, I> {
    fn hash<Ha: Hasher>(&self, state: &mut Ha) {
        self.route.hash(state);
        self.rest_kind.hash(state);
        self.description.hash(state);
    }
}

impl Route<2, 2> {
    /// Creates a new [`Route`] through a REST `GET` API.
    #[must_use]
    pub const fn get(route: &'static str) -> Self {
        Self::init(RestKind::Get, route)
    }

    /// Creates a new [`Route`] through a REST `PUT` API.
    #[must_use]
    pub const fn put(route: &'static str) -> Self {
        Self::init(RestKind::Put, route)
    }

    /// Creates a new [`Route`] through a REST `POST` API.
    #[must_use]
    pub const fn post(route: &'static str) -> Self {
        Self::init(RestKind::Post, route)
    }

    /// Creates a new [`Route`] through a REST `DELETE` API.
    #[must_use]
    pub const fn delete(route: &'static str) -> Self {
        Self::init(RestKind::Delete, route)
    }

    const fn init(rest_kind: RestKind, route: &'static str) -> Self {
        Route::<2, 2> {
            route,
            rest_kind,
            description: None,
            hazards: Hazards::empty(),
            inputs: Inputs::empty(),
        }
    }
}

impl<const H: usize, const I: usize> Route<H, I> {
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
    pub fn with_hazards<const H2: usize>(self, hazards: Hazards<H2>) -> Route<H2, I> {
        Route::<H2, I> {
            route: self.route,
            rest_kind: self.rest_kind,
            description: self.description,
            hazards,
            inputs: self.inputs,
        }
    }

    /// Adds [`Input`] array to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_inputs<const I2: usize>(self, inputs: Inputs<I2>) -> Route<H, I2> {
        Route::<H, I2> {
            route: self.route,
            rest_kind: self.rest_kind,
            description: self.description,
            hazards: self.hazards,
            inputs,
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

    /// Returns [`Inputs`].
    #[must_use]
    pub const fn inputs(&self) -> &Inputs<I> {
        &self.inputs
    }

    /// Serializes [`Route`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> RouteConfig<H, I> {
        RouteConfig::new(self)
    }
}

/// A collection of [`Route`]s.
///
/// **For alignment reasons, it accepts only a power of two
/// as number of elements.**
pub type Routes<const I: usize, const H: usize, const N: usize> = Collection<Route<I, H>, N>;

#[cfg(test)]
mod tests {
    use ascot_library::hazards::Hazard;
    use serde_json::json;

    use crate::input::Input;
    use crate::serialize;

    use super::{Hazards, Inputs, Route};

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
                    .with_hazards(
                        Hazards::<4>::empty()
                            .insert(Hazard::FireHazard)
                            .insert(Hazard::AirPoisoning)
                            .insert(Hazard::Explosion)
                    )
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
    fn test_all_inputs() {
        let expected = json!({
            "name": "/route",
            "description": "A GET route",
            "REST kind": "Get",
            "response kind": "Ok",
            "inputs": [
                {
                    "name": "rangeu64",
                    "structure": {
                        "RangeU64": {
                            "min": 0,
                            "max": 20,
                            "step": 1,
                            "default": 5
                        }
                    }
                },
                {
                    "name": "rangef64",
                    "structure": {
                        "RangeF64": {
                            "min": 0.0,
                            "max": 20.0,
                            "step": 0.1,
                            "default": 0.0
                        }
                    }
                },
                {
                    "name": "bool",
                    "structure": {
                        "Bool": {
                            "default": true,
                        }
                    }
                }
            ],
            "REST kind": "Get"
        });

        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_inputs(
                        Inputs::<4>::empty()
                            .insert(Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5))
                            .insert(Input::rangef64("rangef64", (0., 20., 0.1)))
                            .insert(Input::bool("bool", true))
                    )
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
            "inputs": [
                {
                    "name": "rangeu64",
                    "structure": {
                        "RangeU64": {
                            "min": 0,
                            "max": 20,
                            "step": 1,
                            "default": 5
                        }
                    }
                },
                {
                    "name": "rangef64",
                    "structure": {
                        "RangeF64": {
                            "min": 0.0,
                            "max": 20.0,
                            "step": 0.1,
                            "default": 0.0
                        }
                    }
                },
                {
                    "name": "bool",
                    "structure": {
                        "Bool": {
                            "default": true,
                        }
                    }
                }
            ],
            "REST kind": "Get"
        });

        assert_eq!(
            serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazards(
                        Hazards::<4>::empty()
                            .insert(Hazard::FireHazard)
                            .insert(Hazard::AirPoisoning)
                            .insert(Hazard::Explosion)
                    )
                    .with_inputs(
                        Inputs::<4>::empty()
                            .insert(Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5))
                            .insert(Input::rangef64("rangef64", (0., 20., 0.1)))
                            .insert(Input::bool("bool", true))
                    )
                    .serialize_data()
            ),
            expected
        );
    }
}
