use core::result::Result;

use ascot::device::{DeviceData, DeviceEnvironment, DeviceKind};
use ascot::hazards::Hazard;
use ascot::route::{Route, RouteConfig, RouteConfigs};

use esp_idf_svc::http::server::{EspHttpConnection, Request, Response};
use esp_idf_svc::io::EspIOError;

// Default main route for a device.
const DEFAULT_MAIN_ROUTE: &str = "/device";

// An internal module to avoid declaring the trait as public.
mod internal {
    use super::{EspHttpConnection, EspIOError, Request, Response};
    // A trait to avoid writing over and over the same definition across
    // functions.
    pub trait ResponseTrait:
        for<'a, 'r> Fn(
            Request<&'a mut EspHttpConnection<'r>>,
        ) -> Result<Response<&'a mut EspHttpConnection<'r>>, EspIOError>
        + Send
        + 'static
    {
    }
}

impl<T> internal::ResponseTrait for T where
    T: for<'a, 'r> Fn(
            Request<&'a mut EspHttpConnection<'r>>,
        ) -> Result<Response<&'a mut EspHttpConnection<'r>>, EspIOError>
        + Send
        + 'static
{
}

/// Constructs a response which the server returns whenever the associated
/// action is being invoked.
pub struct ResponseBuilder<R: internal::ResponseTrait>(
    /// The closure called by a server whenever an action is called.
    /// It is responsible for showing the response content.
    pub R,
    /// Response content.
    pub &'static str,
);

/// A device action connects a server route with a device handler and,
/// optionally, with every possible hazards associated with the handler.
pub struct DeviceAction {
    // Route and hazards.
    pub(crate) route_config: RouteConfig,
    // Body.
    pub(crate) body: Option<Box<dyn Fn() -> Result<(), EspIOError> + Send + 'static>>,
    // Response.
    pub(crate) response: Box<dyn internal::ResponseTrait>,
    // Response content.
    pub(crate) content: &'static str,
}

impl DeviceAction {
    /// Creates a [`DeviceAction`].
    #[must_use]
    #[inline]
    pub fn new<R: internal::ResponseTrait>(route: Route, response: ResponseBuilder<R>) -> Self {
        Self {
            route_config: route.serialize_data(),
            body: None,
            response: Box::new(response.0),
            content: response.1,
        }
    }

    /// Checks whether a [`DeviceAction`] misses a specific [`Hazard`].
    #[must_use]
    #[inline]
    pub fn miss_hazard(&self, hazard: Hazard) -> bool {
        !self.route_config.data.hazards.contains(&hazard)
    }

    /// Checks whether a [`DeviceAction`] misses the given [`Hazard`]s.
    #[must_use]
    #[inline]
    pub fn miss_hazards(&self, hazards: &'static [Hazard]) -> bool {
        !hazards
            .iter()
            .all(|hazard| self.route_config.data.hazards.contains(hazard))
    }

    /// Adds the body necessary to construct the response of an action.
    #[must_use]
    #[inline]
    pub fn body<B>(mut self, body: B) -> Self
    where
        B: Fn() -> Result<(), EspIOError> + Send + 'static,
    {
        self.body = Some(Box::new(body));
        self
    }
}

// Build a device from a precise device.
pub(crate) trait DeviceBuilder {
    fn into_device(self) -> Device;
}

/// A general smart home device.
pub struct Device {
    // Kind.
    kind: DeviceKind,
    // Main device route.
    main_route: &'static str,
    // All device routes with their hazards and handlers.
    routes_data: Vec<DeviceAction>,
    // Number of mandatory routes.
    mandatory_routes: u8
}

impl Device {
    /// Creates a new [`Device`] instance.
    #[must_use]
    pub const fn new(kind: DeviceKind) -> Self {
        Self {
            kind,
            main_route: DEFAULT_MAIN_ROUTE,
            routes_data: Vec::new(),
            mandatory_routes: 0,
        }
    }

    /// Sets a new main route.
    #[must_use]
    pub const fn main_route(mut self, main_route: &'static str) -> Self {
        self.main_route = main_route;
        self
    }

    /// Adds a [`DeviceAction`].
    #[must_use]
    pub fn add_action(mut self, device_action: DeviceAction) -> Self {
        self.routes_data.push(device_action);
        self
    }

    /// Sets number of mandatory actions.
    #[must_use]
    pub const fn mandatory_routes(mut self, mandatory_routes: u8) -> Self {
        self.mandatory_routes = mandatory_routes;
        self
    }

    pub(crate) fn finalize(self) -> (&'static str, DeviceData, Vec<DeviceAction>) {
        // TODO: Decouple Router and action information.
        let mut route_configs = RouteConfigs::new();
        for route_data in &self.routes_data {
            route_configs.add(route_data.route_config.clone());
        }

        (
            self.main_route,
            DeviceData::new(
                self.kind,
                DeviceEnvironment::Esp32,
                self.main_route,
                route_configs,
                self.mandatory_routes
            ),
            self.routes_data,
        )
    }
}
