macro_rules! mandatory_route {
    (
        $name:ident,
        $path:expr,
        methods: [$($method:ident),* $(,)?]
    ) => {
        #[doc = concat!("A mandatory [`", stringify!($name), "`].")]
        #[derive(Debug)]
        pub struct $name {
            route: Route,
        }

        impl $name {
            $(
                mandatory_route!(@method_fn $method, $name, $path);
            )*

            #[doc = "Sets the route description."]
            #[must_use]
            pub fn description(mut self, description: &'static str) -> Self {
                self.route = self.route.description(description);
                self
            }

            #[doc = "Changes the route name."]
            #[must_use]
            pub fn change_name(mut self, name: &'static str) -> Self {
                self.route = self.route.change_name(name);
                self
            }

            #[doc = concat!("Adds [`Hazards`] to a [`", stringify!($name), "`].")]
            #[must_use]
            #[inline]
            pub fn with_hazards(mut self, hazards: Hazards) -> Self {
                self.route = self.route.with_hazards(hazards);
                self
            }

            #[doc = concat!("Adds an [`Hazard`] to a [`", stringify!($name), "`].")]
            #[must_use]
            #[inline]
            pub fn with_hazard(mut self, hazard: Hazard) -> Self {
                self.route = self.route.with_hazard(hazard);
                self
            }

            #[doc = concat!("Adds a slice of [`Hazards`] to a [`", stringify!($name), "`].")]
            #[must_use]
            #[inline]
            pub fn with_slice_hazards(mut self, hazards: &'static [Hazard]) -> Self {
                self.route = self.route.with_slice_hazards(hazards);
                self
            }

            #[doc = concat!("Adds [`Parameters`] to a [`", stringify!($name), "`].")]
            #[must_use]
            #[inline]
            pub fn with_parameters(mut self, parameters: Parameters) -> Self {
                self.route = self.route.with_parameters(parameters);
                self
            }

            #[doc = "Returns the route path"]
            #[must_use]
            pub fn route(&self ) -> &str {
                self.route.route()
            }

            #[doc = concat!("Returns [`RestKind`].")]
            #[must_use]
            pub const fn kind(&self) -> RestKind {
                self.route.kind()
            }

            #[doc = concat!("Returns [`Hazards`].")]
            #[must_use]
            pub const fn hazards(&self) -> &Hazards {
               self.route.hazards()
            }

            #[doc = concat!("Returns [`Parameters`].")]
            #[must_use]
            pub const fn parameters(&self) -> &Parameters {
                self.route.parameters()
            }

            #[doc = "Returns the internal [`Route`] instance."]
            #[must_use]
            pub fn into_route(self) -> Route {
                self.route
            }
        }
    };

    (@method_fn get, $name:ident, $path:expr) => {
        #[doc = concat!("Creates a new [`", stringify!($name), "`] through a REST `GET` API.")]
        #[must_use]
        #[inline]
        pub fn get(name: &'static str) -> Self {
            Self {
                route: Route::get(name, $path),
            }
        }
    };

    (@method_fn put, $name:ident, $path:expr) => {
        #[doc = concat!("Creates a new [`", stringify!($name), "`] through a REST `PUT` API.")]
        #[must_use]
        #[inline]
        pub fn put(name: &'static str) -> Self {
            Self {
                route: Route::put(name, $path),
            }
        }
    };

    (@method_fn post, $name:ident, $path:expr) => {
        #[doc = concat!("Creates a new [`", stringify!($name), "`] through a REST `POST` API.")]
        #[must_use]
        #[inline]
        pub fn post(name: &'static str) -> Self {
            Self {
                route: Route::post(name, $path),
            }
        }
    };

    (@method_fn delete, $name:ident, $path:expr) => {
        #[doc = concat!("Creates a new [`", stringify!($name), "`] through a REST `DELETE` API.")]
        #[must_use]
        #[inline]
        pub fn delete(name: &'static str) -> Self {
            Self {
                route: Route::delete(name, $path),
            }
        }
    };
}

pub(crate) use mandatory_route;
