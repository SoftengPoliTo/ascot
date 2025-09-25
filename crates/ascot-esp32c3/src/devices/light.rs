use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use ascot::device::{DeviceData, DeviceEnvironment, DeviceKind};
use ascot::hazards::Hazard;
use ascot::route::{Route, RouteConfigs};

use esp_wifi::wifi::WifiDevice;

use log::error;

use crate::device::Device;
use crate::response::{IntoResponse, JsonResponse, Response};
use crate::server::{FuncIndex, FuncType, InputFn, InputStateFn};
use crate::state::{State, ValueFromRef};

// Default main route.
const MAIN_ROUTE: &str = "/light";

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

// TODO: Check hazards (import this functionality in ascot)
// Return an error if action hazards are not a subset of allowed hazards.
/*for hazard in route.hazards() {
    if !ALLOWED_HAZARDS.contains(hazard) {
        return Err(Error::new(ErrorKind::Device, "Hazard not allowed"));
    }
}*/

/// A `light` device.
///
/// The first placeholder to construct a [`CompleteLight`].
pub struct Light<S = ()>(CompleteLight<S>)
where
    S: ValueFromRef + Send + Sync + 'static;

impl Light<()> {
    /// Creates a [`Light`] without a [`State`].
    #[must_use]
    #[inline]
    pub fn new(wifi_interface: &WifiDevice<'_>) -> Self {
        Self(CompleteLight::with_state(wifi_interface, ()))
    }
}

impl<S> Light<S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    /// Creates a [`Light`] with a [`State`].
    #[inline]
    pub fn with_state(wifi_interface: &WifiDevice<'_>, state: S) -> Self {
        Self(CompleteLight::with_state(wifi_interface, state))
    }

    /// Turns a light on using a stateless handler.
    #[must_use]
    #[inline]
    pub fn turn_light_on_stateless<F, Fut>(
        self,
        route: ascot::route::LightOnRoute,
        func: F,
    ) -> LightOnRoute<S>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        LightOnRoute(self.0.stateless_route(route.into_route(), func))
    }

    /// Turns a light on using a stateful handler.
    #[must_use]
    #[inline]
    pub fn turn_light_on_stateful<F, Fut>(
        self,
        route: ascot::route::LightOnRoute,
        func: F,
    ) -> LightOnRoute<S>
    where
        F: Fn(State<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        LightOnRoute(self.0.stateful_route(route.into_route(), func))
    }
}

/// A `light` placeholder containing only the route to turn the light on.
///
/// All of its methods constructs a [`CompleteLight`].
pub struct LightOnRoute<S = ()>(CompleteLight<S>)
where
    S: ValueFromRef + Send + Sync + 'static;

impl<S> LightOnRoute<S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    /// Turns a light off using a stateless handler.
    #[must_use]
    #[inline]
    pub fn turn_light_off_stateless<F, Fut>(
        self,
        route: ascot::route::LightOffRoute,
        func: F,
    ) -> CompleteLight<S>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        self.0.stateless_route(route.into_route(), func)
    }

    /// Turns a light off using a stateful handler.
    #[must_use]
    #[inline]
    pub fn turn_light_off_stateful<F, Fut>(
        self,
        route: ascot::route::LightOffRoute,
        func: F,
    ) -> CompleteLight<S>
    where
        F: Fn(State<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        self.0.stateful_route(route.into_route(), func)
    }
}

/// A `light` device with methods to turn a light on and off.
pub struct CompleteLight<S = ()>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    main_route: &'static str,
    state: State<S>,
    routes_functions: (Vec<InputFn>, Vec<InputStateFn<S>>),
    device: DeviceData,
    index_array: Vec<FuncIndex>,
}

impl<S> CompleteLight<S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    /// Changes the main route.
    #[must_use]
    #[inline]
    pub fn main_route(mut self, main_route: &'static str) -> Self {
        self.main_route = main_route;
        self.device.main_route = Cow::Borrowed(main_route);
        self
    }

    /// Adds a [`Route`] with a stateless handler.
    #[must_use]
    pub fn stateless_route<F, Fut>(mut self, route: Route, func: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        let route_config = route.serialize_data();

        if self.device.route_configs.contains(&route_config) {
            error!(
                "The route with prefix `{}` already exists!",
                route_config.data.path
            );
        }

        let func: InputFn = Box::new(move || Box::pin(func()));
        self.routes_functions.0.push(func);
        self.device.route_configs.add(route_config);
        self.index_array.push(FuncIndex::new(
            FuncType::First,
            self.routes_functions.0.len() - 1,
        ));
        self
    }

    /// Adds a [`Route`] with a stateful handler.
    #[must_use]
    pub fn stateful_route<F, Fut>(mut self, route: Route, func: F) -> Self
    where
        F: Fn(State<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + Sync + 'static,
    {
        let route_config = route.serialize_data();

        if self.device.route_configs.contains(&route_config) {
            error!(
                "The route with prefix `{}` already exists!",
                route_config.data.path
            );
        }

        let func: InputStateFn<S> = Box::new(move |state| Box::pin(func(state)));
        self.routes_functions.1.push(func);
        self.device.route_configs.add(route_config);
        self.index_array.push(FuncIndex::new(
            FuncType::Second,
            self.routes_functions.1.len() - 1,
        ));
        self
    }

    /// Builds a [`Device`].
    #[must_use]
    #[inline]
    pub fn build(self) -> Device<S> {
        Device::new(
            self.main_route,
            self.state,
            self.routes_functions,
            self.index_array,
            JsonResponse::new(&self.device).into_response(),
            self.device.route_configs,
        )
    }

    #[inline]
    fn with_state(wifi_interface: &WifiDevice<'_>, state: S) -> Self {
        let id = wifi_interface.mac_address();

        let device = DeviceData::new(
            DeviceKind::Light,
            DeviceEnvironment::Esp32,
            MAIN_ROUTE,
            RouteConfigs::new(),
            Some(id),
            None,
            2,
        )
        .description("A light device.");

        Self {
            main_route: MAIN_ROUTE,
            state: State(state),
            routes_functions: (Vec::new(), Vec::new()),
            device,
            index_array: Vec::new(),
        }
    }
}
