use ascot::device::DeviceKind;
use ascot::hazards::Hazard;

use crate::actions::{DeviceAction, MandatoryAction};
use crate::device::Device;
use crate::error::{Error, Result};

// The default main route for a light.
const LIGHT_MAIN_ROUTE: &str = "/light";

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

/// A smart home light.
///
/// The default server main route for a light is `light`.
///
/// If a smart home needs more lights, each light **MUST** provide a
/// **different** main route in order to be registered.
pub struct Light<const M1: bool, const M2: bool, S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    // Internal device.
    device: Device<S>,
    // Turn light on action.
    turn_light_on: MandatoryAction<M1>,
    // Turn light off action.
    turn_light_off: MandatoryAction<M2>,
    // Allowed light hazards.
    allowed_hazards: &'static [Hazard],
}

impl Default for Light<false, false, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Light<false, false, ()> {
    /// Creates a [`Light`] instance without a state.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_state(())
    }
}

impl<S> Light<false, false, S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a [`Light`] instance with a state.
    #[inline]
    pub fn with_state(state: S) -> Self {
        // Create a new device.
        let device = Device::init(DeviceKind::Light, state).main_route(LIGHT_MAIN_ROUTE);

        Self {
            device,
            turn_light_on: MandatoryAction::empty(),
            turn_light_off: MandatoryAction::empty(),
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }

    /// Adds a turn light on action for a [`Light`].
    ///
    /// **This method is mandatory, if not called, a compilation
    /// error is raised.**.
    pub fn turn_light_on(
        self,
        turn_light_on: impl FnOnce(S) -> MandatoryAction<false>,
    ) -> Light<true, false, S> {
        let turn_light_on = turn_light_on(self.device.state.clone());

        Light {
            device: self.device,
            turn_light_on: MandatoryAction::init(turn_light_on.device_action),
            turn_light_off: self.turn_light_off,
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }
}

impl<S> Light<true, false, S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Adds a turn light off action for a [`Light`].
    ///
    ///
    /// **This method is mandatory, if not called, a compilation
    /// error is raised.**.
    pub fn turn_light_off(
        self,
        turn_light_off: impl FnOnce(S) -> MandatoryAction<false>,
    ) -> Light<true, true, S> {
        let turn_light_off = turn_light_off(self.device.state.clone());

        Light {
            device: self.device,
            turn_light_on: self.turn_light_on,
            turn_light_off: MandatoryAction::init(turn_light_off.device_action),
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }
}

impl<S> Light<true, true, S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Sets a new main route.
    #[must_use]
    #[inline]
    pub fn main_route(mut self, main_route: &'static str) -> Self {
        self.device = self.device.main_route(main_route);
        self
    }

    /// Adds an additional action for a [`Light`].
    ///
    /// # Errors
    ///
    /// It returns an error whether one or more hazards are not allowed for
    /// the [`Light`] device.
    pub fn add_action(mut self, light_action: impl FnOnce(S) -> DeviceAction) -> Result<Self> {
        let light_action = light_action(self.device.state.clone());

        // Return an error if action hazards are not a subset of allowed hazards.
        for hazard in light_action.hazards() {
            if !self.allowed_hazards.contains(hazard) {
                return Err(Error::device(
                    DeviceKind::Light,
                    format!("{hazard} hazard is not allowed for light"),
                ));
            }
        }

        self.device = self.device.add_device_action(light_action);

        Ok(self)
    }

    /// Adds an informative action for [`Light`].
    #[must_use]
    pub fn add_info_action(
        mut self,
        light_info_action: impl FnOnce(S, ()) -> DeviceAction,
    ) -> Self {
        let light_info_action = light_info_action(self.device.state.clone(), ());

        self.device = self.device.add_device_action(light_info_action);

        self
    }

    /// Converts a [`Light`] into a [`Device`].
    pub fn into_device(self) -> Device<S> {
        self.device
            .add_device_action(self.turn_light_on.device_action)
            .add_device_action(self.turn_light_off.device_action)
    }
}

#[cfg(test)]
mod tests {

    use ascot::hazards::Hazard;
    use ascot::parameters::Parameters;
    use ascot::route::Route;

    use axum::extract::{Json, State};

    use serde::{Deserialize, Serialize};

    use crate::actions::error::ErrorResponse;
    use crate::actions::ok::{
        OkResponse, mandatory_ok_stateful, mandatory_ok_stateless, ok_stateful, ok_stateless,
    };
    use crate::actions::serial::{
        SerialResponse, mandatory_serial_stateful, mandatory_serial_stateless, serial_stateful,
        serial_stateless,
    };

    use super::Light;

    #[derive(Clone)]
    struct LightState;

    #[derive(Deserialize)]
    struct Inputs {
        brightness: f64,
        #[serde(alias = "save-energy")]
        save_energy: bool,
    }

    #[derive(Serialize, Deserialize)]
    struct LightOnResponse {
        brightness: f64,
        #[serde(rename = "save-energy")]
        save_energy: bool,
    }

    async fn turn_light_on(
        State(_state): State<LightState>,
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<LightOnResponse>, ErrorResponse> {
        Ok(SerialResponse::new(LightOnResponse {
            brightness: inputs.brightness,
            save_energy: inputs.save_energy,
        }))
    }

    async fn turn_light_on_stateless(
        Json(inputs): Json<Inputs>,
    ) -> Result<SerialResponse<LightOnResponse>, ErrorResponse> {
        Ok(SerialResponse::new(LightOnResponse {
            brightness: inputs.brightness,
            save_energy: inputs.save_energy,
        }))
    }

    async fn turn_light_off(State(_state): State<LightState>) -> Result<OkResponse, ErrorResponse> {
        Ok(OkResponse::ok())
    }

    async fn turn_light_off_stateless() -> Result<OkResponse, ErrorResponse> {
        Ok(OkResponse::ok())
    }

    async fn toggle(State(_state): State<LightState>) -> Result<OkResponse, ErrorResponse> {
        Ok(OkResponse::ok())
    }

    async fn toggle_stateless() -> Result<OkResponse, ErrorResponse> {
        Ok(OkResponse::ok())
    }

    struct Routes {
        light_on: Route,
        light_on_post: Route,
        light_off: Route,
        toggle: Route,
    }

    #[inline]
    fn create_routes() -> Routes {
        Routes {
            light_on: Route::put("On", "/on")
                .description("Turn light on.")
                .with_hazard(Hazard::ElectricEnergyConsumption)
                .with_parameters(
                    Parameters::new()
                        .rangef64("brightness", (0., 20., 0.1))
                        .bool("save-energy", false),
                ),

            light_on_post: Route::post("On", "/on")
                .description("Turn light on.")
                .with_hazard(Hazard::ElectricEnergyConsumption)
                .with_parameters(
                    Parameters::new()
                        .rangef64("brightness", (0., 20., 0.1))
                        .bool("save-energy", false),
                ),

            light_off: Route::put("Off", "/off").description("Turn light off."),

            toggle: Route::put("Toggle", "/toggle")
                .description("Toggle a light.")
                .with_hazard(Hazard::ElectricEnergyConsumption),
        }
    }

    #[test]
    fn complete_with_state() {
        let routes = create_routes();

        Light::with_state(LightState {})
            .turn_light_on(mandatory_serial_stateful(routes.light_on, turn_light_on))
            .turn_light_off(mandatory_ok_stateful(routes.light_off, turn_light_off))
            .add_action(serial_stateful(routes.light_on_post, turn_light_on))
            .unwrap()
            .add_action(ok_stateful(routes.toggle, toggle))
            .unwrap()
            .into_device();
    }

    #[test]
    fn without_action_with_state() {
        let routes = create_routes();

        Light::with_state(LightState {})
            .turn_light_on(mandatory_serial_stateful(routes.light_on, turn_light_on))
            .turn_light_off(mandatory_ok_stateful(routes.light_off, turn_light_off))
            .into_device();
    }

    #[test]
    fn stateless_action_with_state() {
        let routes = create_routes();

        Light::with_state(LightState {})
            .turn_light_on(mandatory_serial_stateful(routes.light_on, turn_light_on))
            .turn_light_off(mandatory_ok_stateful(routes.light_off, turn_light_off))
            .add_action(serial_stateful(routes.light_on_post, turn_light_on))
            .unwrap()
            .add_action(ok_stateless(routes.toggle, toggle_stateless))
            .unwrap()
            .into_device();
    }

    #[test]
    fn complete_without_state() {
        let routes = create_routes();

        Light::new()
            .turn_light_on(mandatory_serial_stateless(
                routes.light_on,
                turn_light_on_stateless,
            ))
            .turn_light_off(mandatory_ok_stateless(
                routes.light_off,
                turn_light_off_stateless,
            ))
            .add_action(serial_stateless(
                routes.light_on_post,
                turn_light_on_stateless,
            ))
            .unwrap()
            .add_action(ok_stateless(routes.toggle, toggle_stateless))
            .unwrap()
            .into_device();
    }

    #[test]
    fn without_action_and_state() {
        let routes = create_routes();

        Light::new()
            .turn_light_on(mandatory_serial_stateless(
                routes.light_on,
                turn_light_on_stateless,
            ))
            .turn_light_off(mandatory_ok_stateless(
                routes.light_off,
                turn_light_off_stateless,
            ))
            .into_device();
    }
}
