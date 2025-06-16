use ascot::device::DeviceKind;
use ascot::hazards::Hazard;

use crate::device::{Device, DeviceAction, DeviceBuilder};
use crate::error::{Error, ErrorKind, Result};

// The default main route for a light.
const LIGHT_MAIN_ROUTE: &str = "/light";

// Allowed hazards.
const ALLOWED_HAZARDS: &[Hazard] = &[Hazard::FireHazard, Hazard::ElectricEnergyConsumption];

// Number of mandatory routes.
const MANDATORY_ROUTES: u8 = 2;

/// A smart home light.
///
/// The default server main route for a light is `light`.
///
/// If a smart home needs more lights, each light **MUST** provide a
/// **different** main route in order to be registered.
pub struct Light {
    // Main device route.
    main_route: &'static str,
    // Device.
    device: Device,
    // Allowed light hazards.
    allowed_hazards: &'static [Hazard],
    // Number of mandatory routes.
    mandatory_routes: u8,
}

impl DeviceBuilder for Light {
    fn into_device(self) -> Device {
        self.device
            .main_route(self.main_route)
            .mandatory_routes(self.mandatory_routes)
    }
}

impl Light {
    /// Creates a [`Light`].
    #[must_use]
    pub fn new(turn_light_on: DeviceAction, turn_light_off: DeviceAction) -> Self {
        // Create a new device.
        let device = Device::new(DeviceKind::Light)
            .add_action(turn_light_on)
            .add_action(turn_light_off);

        Self {
            main_route: LIGHT_MAIN_ROUTE,
            device,
            allowed_hazards: ALLOWED_HAZARDS,
            mandatory_routes: MANDATORY_ROUTES,
        }
    }

    /// Sets a new main route.
    #[must_use]
    pub const fn main_route(mut self, main_route: &'static str) -> Self {
        self.main_route = main_route;
        self
    }

    /// Adds an additional action for a [`Light`].
    ///
    /// # Errors
    ///
    /// It returns an error whether one or more hazards are not allowed for
    /// the [`Light`] device.
    pub fn add_action(mut self, light_action: DeviceAction) -> Result<Self> {
        // Return an error if action hazards are not a subset of allowed hazards.
        for hazard in &light_action.route_config.data.hazards {
            if !self.allowed_hazards.contains(hazard) {
                return Err(Error::new(
                    ErrorKind::Light,
                    format!("{hazard} hazard is not allowed for light"),
                ));
            }
        }

        self.device = self.device.add_action(light_action);

        Ok(self)
    }

    /// Builds a new [`Device`].
    #[must_use]
    #[inline]
    pub fn build(self) -> Device {
        self.into_device()
    }
}
