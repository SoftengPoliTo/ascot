use core::hash::{Hash, Hasher};

use ascot_library::hazards::Hazard;
use ascot_library::response::ResponseKind;
use ascot_library::route::RestKind;

use serde::Serialize;

use crate::hazards::Hazards;
use crate::input::{Input, Inputs, InputsData};
use crate::utils::collections::{Collection, SerialCollection};

/// Route data.
#[derive(Debug, Clone, Serialize)]
pub struct RouteData<const N: usize> {
    /// Name.
    pub name: &'static str,
    /// Description.
    pub description: Option<&'static str>,
    /// Hazards data.
    #[serde(skip_serializing_if = "Hazards::is_empty")]
    #[serde(default = "Hazards::empty")]
    pub hazards: Hazards<N>,
    /// Inputs associated with a route..
    #[serde(skip_serializing_if = "InputsData::is_empty")]
    #[serde(default = "InputsData::empty")]
    pub inputs: InputsData<N>,
}

impl<const N: usize> PartialEq for RouteData<N> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl<const N: usize> RouteData<N> {
    fn new(route: Route<N>) -> Self {
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
pub struct RouteConfig<const N: usize> {
    /// Route.
    #[serde(flatten)]
    pub data: RouteData<N>,
    /// **_REST_** kind..
    #[serde(rename = "REST kind")]
    pub rest_kind: RestKind,
    /// Response kind.
    #[serde(rename = "response kind")]
    pub response_kind: ResponseKind,
}

/// A collection of [`RouteConfig`]s.
pub type RouteConfigs<const N: usize> = SerialCollection<RouteConfig<N>, N>;

impl<const N: usize> PartialEq for RouteConfig<N> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data) && self.rest_kind == other.rest_kind
    }
}

impl<const N: usize> Eq for RouteConfig<N> {}

impl<const N: usize> Hash for RouteConfig<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.name.hash(state);
        self.rest_kind.hash(state);
    }
}

impl<const N: usize> RouteConfig<N> {
    fn new(route: Route<N>) -> Self {
        Self {
            rest_kind: route.rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData::new(route),
        }
    }
}

/// A server route.
///
/// It represents a specific `REST` API which, when invoked, runs a task on
/// a remote device.
#[derive(Debug)]
pub struct Route<const N: usize> {
    // Route.
    route: &'static str,
    // REST kind.
    rest_kind: RestKind,
    // Description.
    description: Option<&'static str>,
    // Inputs.
    inputs: Inputs<N>,
    // Hazards.
    hazards: Hazards<N>,
}

impl<const N: usize> PartialEq for Route<N> {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.rest_kind == other.rest_kind
    }
}

impl<const N: usize> Eq for Route<N> {}

impl<const N: usize> Hash for Route<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.route.hash(state);
        self.rest_kind.hash(state);
    }
}

impl Route<2> {
    /// Creates a new [`Route`] through a REST `GET` API.
    #[must_use]
    #[inline]
    pub fn get(route: &'static str) -> Self {
        Self::init(RestKind::Get, route)
    }

    /// Creates a new [`Route`] through a REST `PUT` API.
    #[must_use]
    #[inline]
    pub fn put(route: &'static str) -> Self {
        Self::init(RestKind::Put, route)
    }

    /// Creates a new [`Route`] through a REST `POST` API.
    #[must_use]
    #[inline]
    pub fn post(route: &'static str) -> Self {
        Self::init(RestKind::Post, route)
    }

    /// Creates a new [`Route`] through a REST `DELETE` API.
    #[must_use]
    #[inline]
    pub fn delete(route: &'static str) -> Self {
        Self::init(RestKind::Delete, route)
    }

    fn init(rest_kind: RestKind, route: &'static str) -> Self {
        Route::<2> {
            route,
            rest_kind,
            description: None,
            hazards: Hazards::empty(),
            inputs: Inputs::empty(),
        }
    }
}

impl<const N: usize> Route<N> {
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

    /// Adds a single [`Input`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_input(mut self, input: Input) -> Self {
        self.inputs.add(input);
        self
    }

    /// Adds [`Input`] array to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_inputs<const NI: usize>(mut self, inputs: [Input; NI]) -> Self {
        for input in inputs {
            self.inputs.add(input);
        }
        self
    }

    /// Adds [`Hazards`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_hazards(mut self, hazards: Hazards<N>) -> Self {
        self.hazards = hazards;
        self
    }

    /// Adds an [`Hazard`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_hazard(mut self, hazard: Hazard) -> Self {
        self.hazards = Hazards::init(hazard);
        self
    }

    /// Adds a slice of [`Hazard`]s to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_slice_hazards(mut self, hazards: &'static [Hazard]) -> Self {
        self.hazards = Hazards::init_with_elements(hazards);
        self
    }

    /// Returns route.
    #[must_use]
    pub fn route(&self) -> &str {
        self.route
    }

    /// Returns [`RestKind`].
    #[must_use]
    pub const fn kind(&self) -> RestKind {
        self.rest_kind
    }

    /// Returns [`Hazards`].
    #[must_use]
    pub const fn hazards(&self) -> &Hazards<N> {
        &self.hazards
    }

    /// Returns [`Inputs`].
    #[must_use]
    pub const fn inputs(&self) -> &Inputs<N> {
        &self.inputs
    }

    /// Serializes [`Route`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> RouteConfig<N> {
        RouteConfig::new(self)
    }
}

/// A collection of [`Route`]s.
pub type Routes<const N: usize> = Collection<Route<N>, N>;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::{Hazard, Hazards, Input, Route};

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
                Route::<2>::get("/route")
                    .description("A GET route")
                    .with_hazard(Hazard::FireHazard)
                    .serialize_data()
            ),
            json!({
                "name": "/route",
                "description": "A GET route",
                "REST kind": "Get",
                "response kind": "Ok",
                "hazards": [
                    "FireHazard"
                ],
            })
        );

        assert_eq!(
            serialize(
                Route::<2>::get("/route")
                    .description("A GET route")
                    .with_hazards(
                        Hazards::empty()
                            .insert(Hazard::FireHazard)
                            .insert(Hazard::AirPoisoning)
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
                ],
            })
        );

        assert_eq!(
            serialize(
                Route::<2>::get("/route")
                    .description("A GET route")
                    .with_slice_hazards(&[Hazard::FireHazard, Hazard::AirPoisoning])
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
                }
            ],
            "REST kind": "Get"
        });

        assert_eq!(
            serialize(
                Route::<2>::get("/route")
                    .description("A GET route")
                    .with_input(Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5))
                    .with_input(Input::rangef64("rangef64", (0., 20., 0.1)))
                    .serialize_data()
            ),
            expected
        );

        assert_eq!(
            serialize(
                Route::<2>::get("/route")
                    .description("A GET route")
                    .with_inputs([
                        Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5),
                        Input::rangef64("rangef64", (0., 20., 0.1))
                    ])
                    .serialize_data()
            ),
            expected
        );
    }
}
