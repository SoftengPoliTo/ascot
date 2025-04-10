use serde::Serialize;

/// `REST` requests kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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

#[cfg(feature = "alloc")]
mod internal_route {
    use alloc::borrow::Cow;

    use serde::Deserialize;

    use crate::hazards::{Hazard, Hazards};
    use crate::parameters::{Parameters, ParametersData};
    use crate::response::ResponseKind;

    use crate::collections::{OutputSet, Set};

    use super::{RestKind, Serialize};

    /// Route data.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RouteData {
        /// Name.
        pub name: Cow<'static, str>,
        /// Description.
        pub description: Option<Cow<'static, str>>,
        /// Hazards data.
        #[serde(skip_serializing_if = "Hazards::is_empty")]
        #[serde(default = "Hazards::new")]
        pub hazards: Hazards,
        /// Input parameters associated with a route.
        #[serde(skip_serializing_if = "ParametersData::is_empty")]
        #[serde(default = "ParametersData::new")]
        pub parameters: ParametersData,
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
                parameters: route.parameters.serialize_data(),
            }
        }
    }

    /// A server route configuration.
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

    /// A collection of [`RouteConfig`]s.
    pub type RouteConfigs = OutputSet<RouteConfig>;

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

    /// A server route.
    ///
    /// It represents a specific `REST` API which, when invoked, runs a task on
    /// a remote device.
    #[derive(Debug)]
    pub struct Route {
        // Route.
        route: &'static str,
        // REST kind.
        rest_kind: RestKind,
        // Description.
        description: Option<&'static str>,
        // Input route parameters.
        parameters: Parameters,
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

        /// Adds [`Parameters`] to a [`Route`].
        #[must_use]
        #[inline]
        pub fn with_parameters(mut self, parameters: Parameters) -> Self {
            self.parameters = parameters;
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

        /// Returns [`Parameters`].
        #[must_use]
        pub const fn parameters(&self) -> &Parameters {
            &self.parameters
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
                hazards: Hazards::new(),
                parameters: Parameters::new(),
            }
        }
    }

    /// A collection of [`Route`]s.
    pub type Routes = Set<Route>;
}

#[cfg(feature = "alloc")]
pub use internal_route::{Route, RouteConfig, RouteConfigs, RouteData, Routes};

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use crate::hazards::{Hazard, Hazards};
    use crate::parameters::{ParameterKind, Parameters, ParametersData};
    use crate::response::ResponseKind;
    use crate::{deserialize, serialize};

    use super::{RestKind, Route, RouteConfig, RouteData};

    fn route_config_empty(rest_kind: RestKind, desc: &'static str) -> RouteConfig {
        route_config_hazards(rest_kind, Hazards::new(), desc)
    }

    fn route_config_hazards(
        rest_kind: RestKind,
        hazards: Hazards,
        desc: &'static str,
    ) -> RouteConfig {
        route_config_parameters(rest_kind, hazards, desc, ParametersData::new())
    }

    fn route_config_parameters(
        rest_kind: RestKind,
        hazards: Hazards,
        desc: &'static str,
        parameters: ParametersData,
    ) -> RouteConfig {
        RouteConfig {
            rest_kind,
            response_kind: ResponseKind::default(),
            data: RouteData {
                name: "/route".into(),
                description: Some(desc.into()),
                hazards,
                parameters,
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
                Hazards::new().insert(Hazard::FireHazard),
                "A GET route"
            )
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_hazards(
                        Hazards::new()
                            .insert(Hazard::FireHazard)
                            .insert(Hazard::AirPoisoning)
                    )
                    .serialize_data()
            )),
            route_config_hazards(
                RestKind::Get,
                Hazards::new()
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
                Hazards::new()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::AirPoisoning),
                "A GET route"
            )
        );
    }

    #[test]
    fn test_all_parameters() {
        let expected = route_config_parameters(
            RestKind::Get,
            Hazards::new(),
            "A GET route",
            ParametersData::new().insert(
                "rangeu64".into(),
                ParameterKind::RangeU64 {
                    min: 0,
                    max: 20,
                    step: 1,
                    default: 5,
                },
            ),
        );

        assert_eq!(
            deserialize::<RouteConfig>(serialize(
                Route::get("/route")
                    .description("A GET route")
                    .with_parameters(
                        Parameters::new()
                            .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
                            .rangef64("rangef64", (0., 20., 0.1))
                    )
                    .serialize_data()
            )),
            expected
        );
    }
}
