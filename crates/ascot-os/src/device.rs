use ascot::device::{DeviceData, DeviceEnvironment, DeviceKind};
use ascot::route::RouteConfigs;

use axum::Router;

use tracing::info;

use crate::actions::DeviceAction;

// Default main route for a device.
const DEFAULT_MAIN_ROUTE: &str = "/device";

/// A general device.
#[derive(Debug)]
pub struct Device<S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    // Main device route.
    main_route: &'static str,
    // Router.
    router: Router,
    // State.
    pub(crate) state: S,
    // Kind.
    kind: DeviceKind,
    // All device routes and their hazards.
    route_configs: RouteConfigs,
}

impl Default for Device<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Device<()> {
    /// Creates an unknown [`Device`] without a state.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_state(())
    }
}

impl<S> Device<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates an unknown [`Device`] with state.
    #[must_use]
    #[inline]
    pub fn with_state(state: S) -> Self {
        Self::init(DeviceKind::Unknown, state)
    }

    /// Sets a new main route.
    #[must_use]
    pub const fn main_route(mut self, main_route: &'static str) -> Self {
        self.main_route = main_route;
        self
    }

    /// Adds an action to the [`Device`].
    #[must_use]
    #[inline]
    pub fn add_action(self, device_action: impl FnOnce(S) -> DeviceAction) -> Self {
        let device_action = device_action(self.state.clone());
        self.add_device_action(device_action)
    }

    /// Adds an informative action to the [`Device`].
    #[must_use]
    pub fn add_info_action(self, device_info_action: impl FnOnce(S, ()) -> DeviceAction) -> Self {
        let device_info_action = device_info_action(self.state.clone(), ());
        self.add_device_action(device_info_action)
    }

    pub(crate) fn init(kind: DeviceKind, state: S) -> Self {
        Self {
            main_route: DEFAULT_MAIN_ROUTE,
            router: Router::new(),
            kind,
            route_configs: RouteConfigs::new(),
            state,
        }
    }

    pub(crate) fn add_device_action(mut self, device_action: DeviceAction) -> Self {
        self.router = self.router.merge(device_action.router);
        self.route_configs.add(device_action.route_config);
        self
    }

    pub(crate) fn finalize(self) -> (&'static str, DeviceData, Router) {
        for route in &self.route_configs {
            info!(
                "Device route: [{}, \"{}{}\"]",
                route.rest_kind, self.main_route, route.data.name,
            );
        }

        (
            self.main_route,
            DeviceData::new(
                self.kind,
                DeviceEnvironment::Os,
                self.main_route,
                self.route_configs,
            ),
            self.router,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use core::ops::{Deref, DerefMut};

    use ascot::device::DeviceInfo;
    use ascot::energy::Energy;
    use ascot::route::Route;

    use async_lock::Mutex;

    use axum::extract::{FromRef, Json, State};

    use serde::{Deserialize, Serialize};

    use crate::actions::error::ErrorResponse;
    use crate::actions::info::{InfoResponse, info_stateful};
    use crate::actions::serial::{SerialResponse, serial_stateful, serial_stateless};

    use super::Device;

    #[derive(Clone)]
    struct DeviceState<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        state: S,
        info: DeviceInfoState,
    }

    impl DeviceState<()> {
        fn empty() -> Self {
            Self::new(())
        }
    }

    impl<S> DeviceState<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        fn new(state: S) -> Self {
            Self {
                state,
                info: DeviceInfoState::new(DeviceInfo::empty()),
            }
        }

        fn add_device_info(mut self, info: DeviceInfo) -> Self {
            self.info = DeviceInfoState::new(info);
            self
        }
    }

    #[derive(Clone)]
    struct SubState {}

    impl FromRef<DeviceState<SubState>> for SubState {
        fn from_ref(device_state: &DeviceState<SubState>) -> SubState {
            device_state.state.clone()
        }
    }

    #[derive(Clone)]
    struct DeviceInfoState {
        info: Arc<Mutex<DeviceInfo>>,
    }

    impl DeviceInfoState {
        fn new(info: DeviceInfo) -> Self {
            Self {
                info: Arc::new(Mutex::new(info)),
            }
        }
    }

    impl Deref for DeviceInfoState {
        type Target = Arc<Mutex<DeviceInfo>>;

        fn deref(&self) -> &Self::Target {
            &self.info
        }
    }

    impl DerefMut for DeviceInfoState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.info
        }
    }

    impl<S> FromRef<DeviceState<S>> for DeviceInfoState
    where
        S: Clone + Send + Sync + 'static,
    {
        fn from_ref(device_state: &DeviceState<S>) -> DeviceInfoState {
            device_state.info.clone()
        }
    }

    #[derive(Deserialize)]
    struct Inputs {
        parameter: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct DeviceResponse {
        parameter: f64,
    }

    async fn serial_action_with_state(
        State(_state): State<DeviceState<()>>,
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<DeviceResponse>, ErrorResponse> {
        Ok(SerialResponse::new(DeviceResponse {
            parameter: inputs.parameter,
        }))
    }

    async fn serial_action_with_substate1(
        State(_state): State<SubState>,
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<DeviceResponse>, ErrorResponse> {
        Ok(SerialResponse::new(DeviceResponse {
            parameter: inputs.parameter,
        }))
    }

    #[derive(Serialize, Deserialize)]
    struct DeviceInfoResponse {
        parameter: f64,
        device_info: DeviceInfo,
    }

    // This method is just a demonstration of this library flexibility,
    // but we do not recommend it because a DeviceInfo inside a SerialResponse
    // could be ignored as response by a receiver.
    async fn serial_action_with_substate2(
        State(state): State<DeviceInfoState>,
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<DeviceInfoResponse>, ErrorResponse> {
        // Retrieve internal state.
        let mut device_info = state.lock().await;

        // Change state.
        device_info.energy = Energy::empty();

        Ok(SerialResponse::new(DeviceInfoResponse {
            parameter: inputs.parameter,
            device_info: device_info.clone(),
        }))
    }

    async fn info_action_with_substate3(
        State(state): State<DeviceInfoState>,
    ) -> Result<InfoResponse, ErrorResponse> {
        // Retrieve internal state.
        let mut device_info = state.lock().await;

        // Change state.
        device_info.energy = Energy::empty();

        Ok(InfoResponse::new(device_info.clone()))
    }

    async fn serial_action_without_state(
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<DeviceResponse>, ErrorResponse> {
        Ok(SerialResponse::new(DeviceResponse {
            parameter: inputs.parameter,
        }))
    }

    struct AllRoutes {
        with_state_route: Route,
        without_state_route: Route,
    }

    #[inline]
    fn create_routes() -> AllRoutes {
        AllRoutes {
            with_state_route: Route::put("/state-action").description("Run action with state."),

            without_state_route: Route::post("/no-state-route")
                .description("Run action without state."),
        }
    }

    #[test]
    fn with_state() {
        let routes = create_routes();

        let state = DeviceState::empty().add_device_info(DeviceInfo::empty());

        let _ = Device::with_state(state)
            .add_action(serial_stateful(
                routes.with_state_route,
                serial_action_with_state,
            ))
            .add_action(serial_stateless(
                routes.without_state_route,
                serial_action_without_state,
            ));
    }

    #[test]
    fn with_substates() {
        let routes = create_routes();

        let state = DeviceState::new(SubState {}).add_device_info(DeviceInfo::empty());

        let _ = Device::with_state(state)
            .add_action(serial_stateful(
                routes.with_state_route,
                serial_action_with_substate1,
            ))
            .add_action(serial_stateful(
                Route::put("/substate-action").description("Run a serial action with a substate."),
                serial_action_with_substate2,
            ))
            .add_info_action(info_stateful(
                Route::put("/substate-info")
                    .description("Run an informative action with a substate."),
                info_action_with_substate3,
            ))
            .add_action(serial_stateless(
                routes.without_state_route,
                serial_action_without_state,
            ));
    }

    #[test]
    fn without_state() {
        let routes = create_routes();

        let _ = Device::new().add_action(serial_stateless(
            routes.without_state_route,
            serial_action_without_state,
        ));
    }
}
