use serde::{Deserialize, Serialize};

use crate::collections::Collection;
use crate::hazards::{Hazard, Hazards};
use crate::input::{Input, Inputs, InputsData};
use crate::response::ResponseKind;

/// `REST` requests kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestKind {
    /// `GET` request.
    Get,
    /// `PUT` request.
    Put,
    /// `POST` request.
    Post,
    /// `DELETE` request.
    Delete,
}

impl core::fmt::Display for RestKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Get => "GET",
            Self::Put => "PUT",
            Self::Post => "POST",
            Self::Delete => "DELETE",
        }
        .fmt(f)
    }
}

/// Route data.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteData {
    /// Name.
    pub name: alloc::borrow::Cow<'static, str>,
    /// Description.
    pub description: Option<alloc::borrow::Cow<'static, str>>,
    /// Hazards data.
    #[serde(skip_serializing_if = "Hazards::is_empty")]
    #[serde(default = "Hazards::empty")]
    pub hazards: Hazards,
    /// Inputs associated with a route..
    #[serde(skip_serializing_if = "InputsData::is_empty")]
    #[serde(default = "InputsData::empty")]
    pub inputs: InputsData,
}

impl PartialEq for RouteData {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl RouteData {
    fn new(route: Route) -> Self {
        Self {
            name: route.route.into(),
            description: route.description.map(core::convert::Into::into),
            hazards: route.hazards,
            inputs: InputsData::from(route.inputs),
        }
    }
}

/// A server route configuration.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// Route.
    #[serde(flatten)]
    pub data: RouteData,
    /// **_REST_** kind..
    #[serde(rename = "REST kind")]
    pub rest_kind: RestKind,
    /// Response kind.
    #[serde(rename = "response kind")]
    pub response_kind: ResponseKind,
}

impl PartialEq for RouteConfig {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data) && self.rest_kind == other.rest_kind
    }
}

impl Eq for RouteConfig {}

impl core::hash::Hash for RouteConfig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.name.hash(state);
        self.rest_kind.hash(state);
    }
}

impl RouteConfig {
    fn new(route: Route) -> Self {
        Self {
            rest_kind: route.rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData::new(route),
        }
    }
}

/// A collection of [`RouteConfig`]s.
#[cfg(feature = "alloc")]
pub type RouteConfigs = crate::collections::OutputCollection<RouteConfig>;

/// A server route.
///
/// It represents a specific `REST` API which, when invoked, runs a task on
/// a remote device.
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct Route {
    // Route.
    route: &'static str,
    // REST kind.
    rest_kind: RestKind,
    // Description.
    description: Option<&'static str>,
    // Inputs.
    inputs: Inputs,
    // Hazards.
    hazards: Hazards,
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route && self.rest_kind == other.rest_kind
    }
}

impl Eq for Route {}

impl core::hash::Hash for Route {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.route.hash(state);
        self.rest_kind.hash(state);
    }
}

impl Route {
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
    pub fn with_inputs<const N: usize>(mut self, inputs: [Input; N]) -> Self {
        for input in inputs {
            self.inputs.add(input);
        }
        self
    }

    /// Adds [`Hazards`] to a [`Route`].
    #[must_use]
    #[inline]
    pub fn with_hazards(mut self, hazards: Hazards) -> Self {
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
    pub const fn hazards(&self) -> &Hazards {
        &self.hazards
    }

    /// Returns [`Inputs`].
    #[must_use]
    pub const fn inputs(&self) -> &Inputs {
        &self.inputs
    }

    /// Serializes [`Route`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> RouteConfig {
        RouteConfig::new(self)
    }

    fn init(rest_kind: RestKind, route: &'static str) -> Self {
        Self {
            route,
            rest_kind,
            description: None,
            hazards: Hazards::empty(),
            inputs: Inputs::empty(),
        }
    }
}

/// A collection of [`Route`]s.
#[cfg(feature = "alloc")]
pub type Routes = crate::collections::Collection<Route>;

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use crate::input::InputData;
    use crate::{deserialize, serialize};

    use super::{
        Hazard, Hazards, Input, InputsData, ResponseKind, RestKind, Route, RouteConfig, RouteData,
    };

    fn route_config_empty(rest_kind: RestKind, desc: &'static str) -> RouteConfig {
        route_config_hazards(rest_kind, Hazards::empty(), desc)
    }

    fn route_config_hazards(
        rest_kind: RestKind,
        hazards: Hazards,
        desc: &'static str,
    ) -> RouteConfig {
        route_config_inputs(rest_kind, hazards, desc, InputsData::empty())
    }

    fn route_config_inputs(
        rest_kind: RestKind,
        hazards: Hazards,
        desc: &'static str,
        inputs: InputsData,
    ) -> RouteConfig {
        RouteConfig {
            rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData {
                name: "/route".into(),
                description: Some(desc.into()),
                hazards,
                inputs,
            },
        }
    }

    #[test]
    fn test_all_routes() {
        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .serialize_data()
            )),
            route_config_empty(RestKind::Get, "A GET route",)
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::put("/route")
                    .description("A PUT route")
                    .serialize_data()
            )),
            route_config_empty(RestKind::Put, "A PUT route",)
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::post("/route")
                    .description("A POST route")
                    .serialize_data()
            )),
            route_config_empty(RestKind::Post, "A POST route",)
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::delete("/route")
                    .description("A DELETE route")
                    .serialize_data()
            )),
            route_config_empty(RestKind::Delete, "A DELETE route",)
        );
    }

    #[test]
    fn test_all_hazards() {
        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazard(Hazard::FireHazard)
                    .serialize_data()
            )),
            route_config_hazards(
                RestKind::Get,
                Hazards::empty().insert(Hazard::FireHazard),
                "A GET route"
            )
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazards(
                        Hazards::empty()
                            .insert(Hazard::FireHazard)
                            .insert(Hazard::AirPoisoning)
                    )
                    .serialize_data()
            )),
            route_config_hazards(
                RestKind::Get,
                Hazards::empty()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::AirPoisoning),
                "A GET route"
            )
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_slice_hazards(&[Hazard::FireHazard, Hazard::AirPoisoning])
                    .serialize_data()
            )),
            route_config_hazards(
                RestKind::Get,
                Hazards::empty()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::AirPoisoning),
                "A GET route"
            )
        );
    }

    #[test]
    fn test_all_inputs() {
        let expected = route_config_inputs(
            RestKind::Get,
            Hazards::empty(),
            "A GET route",
            InputsData::empty().insert(InputData::from(Input::rangeu64_with_default(
                "rangeu64",
                (0, 20, 1),
                5,
            ))),
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_input(Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5))
                    .with_input(Input::rangef64("rangef64", (0., 20., 0.1)))
                    .serialize_data()
            )),
            expected
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_inputs([
                        Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5),
                        Input::rangef64("rangef64", (0., 20., 0.1))
                    ])
                    .serialize_data()
            )),
            expected
        );
    }
}
