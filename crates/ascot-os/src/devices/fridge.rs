use ascot::device::DeviceKind;
use ascot::hazards::Hazard;

use crate::actions::{DeviceAction, MandatoryAction};
use crate::device::Device;
use crate::error::{Error, Result};

// The default main route for a fridge.
const FRIDGE_MAIN_ROUTE: &str = "/fridge";

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::ElectricEnergyConsumption, Hazard::SpoiledFood];

/// A smart home fridge.
///
/// The default server main route for a fridge is `fridge`.
///
/// If a smart home needs more fridges, each fridge **MUST** provide a
/// **different** main route in order to be registered.
pub struct Fridge<const M1: bool, const M2: bool, S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    // Device.
    device: Device<S>,
    // Increase temperature action.
    increase_temperature: MandatoryAction<M1>,
    // Decrease temperature action.
    decrease_temperature: MandatoryAction<M2>,
    // Allowed fridge hazards.
    allowed_hazards: &'static [Hazard],
}

impl Default for Fridge<false, false, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Fridge<false, false, ()> {
    /// Creates a [`Fridge`] instance without a state.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::with_state(())
    }
}

impl<S> Fridge<false, false, S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a [`Fridge`] instance with a state.
    #[inline]
    pub fn with_state(state: S) -> Self {
        // Create a new device.
        let device = Device::init(DeviceKind::Fridge, state).main_route(FRIDGE_MAIN_ROUTE);

        Self {
            device,
            increase_temperature: MandatoryAction::empty(),
            decrease_temperature: MandatoryAction::empty(),
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }

    /// Adds an increase temperature action for a [`Fridge`].
    ///
    /// **This method is mandatory, if not called, a compilation
    /// error is raised.**.
    pub fn increase_temperature(
        self,
        increase_temperature: impl FnOnce(S) -> MandatoryAction<false>,
    ) -> Fridge<true, false, S> {
        let increase_temperature = increase_temperature(self.device.state.clone());

        Fridge {
            device: self.device,
            increase_temperature: MandatoryAction::init(increase_temperature.device_action),
            decrease_temperature: self.decrease_temperature,
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }
}

impl<S> Fridge<true, false, S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Adds a decrease temperature action for a [`Fridge`].
    ///
    /// **This method is mandatory, if not called, a compilation
    /// error is raised.**.
    pub fn decrease_temperature(
        self,
        decrease_temperature: impl FnOnce(S) -> MandatoryAction<false>,
    ) -> Fridge<true, true, S> {
        let decrease_temperature = decrease_temperature(self.device.state.clone());

        Fridge {
            device: self.device,
            increase_temperature: self.increase_temperature,
            decrease_temperature: MandatoryAction::init(decrease_temperature.device_action),
            allowed_hazards: ALLOWED_HAZARDS,
        }
    }
}

impl<S> Fridge<true, true, S>
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

    /// Adds an additional action for a [`Fridge`].
    ///
    /// # Errors
    ///
    /// It returns an error whether one or more hazards are not allowed for
    /// the [`Fridge`] device.
    pub fn add_action(mut self, fridge_action: impl FnOnce(S) -> DeviceAction) -> Result<Self> {
        let fridge_action = fridge_action(self.device.state.clone());

        // Return an error if action hazards are not a subset of allowed hazards.
        for hazard in fridge_action.hazards() {
            if !self.allowed_hazards.contains(hazard) {
                return Err(Error::device(
                    DeviceKind::Fridge,
                    format!("{hazard} hazard is not allowed for fridge"),
                ));
            }
        }

        self.device = self.device.add_device_action(fridge_action);

        Ok(self)
    }

    /// Adds an informative action for [`Fridge`].
    #[must_use]
    pub fn add_info_action(
        mut self,
        fridge_info_action: impl FnOnce(S, ()) -> DeviceAction,
    ) -> Self {
        let fridge_info_action = fridge_info_action(self.device.state.clone(), ());

        self.device = self.device.add_device_action(fridge_info_action);

        self
    }

    /// Converts a [`Fridge`] into a [`Device`].
    pub fn into_device(self) -> Device<S> {
        self.device
            .add_device_action(self.increase_temperature.device_action)
            .add_device_action(self.decrease_temperature.device_action)
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
    use crate::actions::serial::{
        SerialResponse, mandatory_serial_stateful, mandatory_serial_stateless, serial_stateful,
        serial_stateless,
    };

    use super::Fridge;

    #[derive(Clone)]
    struct FridgeState;

    #[derive(Deserialize)]
    struct IncreaseTemperature {
        increment: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct ChangeTempResponse {
        temperature: f64,
    }

    async fn increase_temperature(
        State(_state): State<FridgeState>,
        Json(inputs): Json<IncreaseTemperature>,
    ) -> Result<SerialResponse<ChangeTempResponse>, ErrorResponse> {
        Ok(SerialResponse::new(ChangeTempResponse {
            temperature: inputs.increment,
        }))
    }

    async fn increase_temperature_without_state(
        Json(inputs): Json<IncreaseTemperature>,
    ) -> Result<SerialResponse<ChangeTempResponse>, ErrorResponse> {
        Ok(SerialResponse::new(ChangeTempResponse {
            temperature: inputs.increment,
        }))
    }

    #[derive(Deserialize)]
    struct DecreaseTemperature {
        decrement: f64,
    }

    async fn decrease_temperature(
        State(_state): State<FridgeState>,
        Json(inputs): Json<DecreaseTemperature>,
    ) -> Result<SerialResponse<ChangeTempResponse>, ErrorResponse> {
        Ok(SerialResponse::new(ChangeTempResponse {
            temperature: inputs.decrement,
        }))
    }

    async fn decrease_temperature_without_state(
        Json(inputs): Json<DecreaseTemperature>,
    ) -> Result<SerialResponse<ChangeTempResponse>, ErrorResponse> {
        Ok(SerialResponse::new(ChangeTempResponse {
            temperature: inputs.decrement,
        }))
    }

    struct Routes {
        increase_temp: Route,
        decrease_temp: Route,
        increase_temp_post: Route,
    }

    #[inline]
    fn create_routes() -> Routes {
        Routes {
            increase_temp: Route::put("/increase-temperature")
                .description("Increase temperature.")
                .with_slice_hazards(&[Hazard::ElectricEnergyConsumption, Hazard::SpoiledFood])
                .with_parameters(Parameters::new().rangef64_with_default(
                    "increment",
                    (1., 4., 0.1),
                    2.,
                )),

            decrease_temp: Route::put("/decrease-temperature")
                .description("Decrease temperature.")
                .with_slice_hazards(&[Hazard::ElectricEnergyConsumption, Hazard::SpoiledFood])
                .with_parameters(Parameters::new().rangef64_with_default(
                    "decrement",
                    (1., 4., 0.1),
                    2.,
                )),

            increase_temp_post: Route::post("/increase-temperature")
                .description("Increase temperature.")
                .with_slice_hazards(&[Hazard::ElectricEnergyConsumption, Hazard::SpoiledFood])
                .with_parameters(Parameters::new().rangef64_with_default(
                    "increment",
                    (1., 4., 0.1),
                    2.,
                )),
        }
    }

    #[test]
    fn complete_with_state() {
        let routes = create_routes();

        Fridge::with_state(FridgeState {})
            .increase_temperature(mandatory_serial_stateful(
                routes.increase_temp,
                increase_temperature,
            ))
            .decrease_temperature(mandatory_serial_stateful(
                routes.decrease_temp,
                decrease_temperature,
            ))
            .add_action(serial_stateful(
                routes.increase_temp_post,
                increase_temperature,
            ))
            .unwrap()
            .into_device();
    }

    #[test]
    fn without_action_with_state() {
        let routes = create_routes();

        Fridge::with_state(FridgeState {})
            .increase_temperature(mandatory_serial_stateful(
                routes.increase_temp,
                increase_temperature,
            ))
            .decrease_temperature(mandatory_serial_stateful(
                routes.decrease_temp,
                decrease_temperature,
            ))
            .into_device();
    }

    #[test]
    fn stateless_action_with_state() {
        let routes = create_routes();

        Fridge::with_state(FridgeState {})
            .increase_temperature(mandatory_serial_stateful(
                routes.increase_temp,
                increase_temperature,
            ))
            .decrease_temperature(mandatory_serial_stateful(
                routes.decrease_temp,
                decrease_temperature,
            ))
            .add_action(serial_stateless(
                routes.increase_temp_post,
                increase_temperature_without_state,
            ))
            .unwrap()
            .into_device();
    }

    #[test]
    fn complete_without_state() {
        let routes = create_routes();

        Fridge::new()
            .increase_temperature(mandatory_serial_stateless(
                routes.increase_temp,
                increase_temperature_without_state,
            ))
            .decrease_temperature(mandatory_serial_stateless(
                routes.decrease_temp,
                decrease_temperature_without_state,
            ))
            .add_action(serial_stateless(
                routes.increase_temp_post,
                increase_temperature_without_state,
            ))
            .unwrap()
            .into_device();
    }

    #[test]
    fn without_action_and_state() {
        let routes = create_routes();

        Fridge::new()
            .increase_temperature(mandatory_serial_stateless(
                routes.increase_temp,
                increase_temperature_without_state,
            ))
            .decrease_temperature(mandatory_serial_stateless(
                routes.decrease_temp,
                decrease_temperature_without_state,
            ))
            .into_device();
    }
}
