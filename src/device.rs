use serde::Serialize;

#[cfg(feature = "alloc")]
use crate::economy::Economy;
#[cfg(feature = "alloc")]
use crate::energy::Energy;
#[cfg(feature = "alloc")]
use crate::route::RouteConfigs;

/// A device kind.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum DeviceKind {
    /// Unknown.
    Unknown,
    /// Light.
    Light,
    /// Fridge.
    Fridge,
    /// Camera.
    Camera,
}

impl DeviceKind {
    const fn description(self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Light => "Light",
            Self::Fridge => "Fridge",
            Self::Camera => "Camera",
        }
    }
}

impl core::fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Device environment.
///
/// Some information about the device environment on which a firmware runs on.
/// It might be an operating system or the name of the underlying hardware
/// architecture.
///
/// This enumerator allows to discriminate the different implementations among
/// the supported architectures on a controller side.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum DeviceEnvironment {
    /// Operating system.
    Os,
    /// Esp32.
    Esp32,
}

/// Device information.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Clone, Serialize, serde::Deserialize)]
pub struct DeviceInfo {
    /// Energy information.
    #[serde(skip_serializing_if = "Energy::is_empty")]
    #[serde(default = "Energy::empty")]
    pub energy: Energy,
    /// Economy information.
    #[serde(skip_serializing_if = "Economy::is_empty")]
    #[serde(default = "Economy::empty")]
    pub economy: Economy,
}

#[cfg(feature = "alloc")]
impl DeviceInfo {
    /// Creates a [`DeviceInfo`].
    #[must_use]
    pub fn empty() -> Self {
        Self {
            energy: Energy::empty(),
            economy: Economy::empty(),
        }
    }

    /// Adds [`Energy`] data.
    #[must_use]
    pub fn add_energy(mut self, energy: Energy) -> Self {
        self.energy = energy;
        self
    }

    /// Adds [`Economy`] data.
    #[must_use]
    pub fn add_economy(mut self, economy: Economy) -> Self {
        self.economy = economy;
        self
    }
}

/// Device data.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
pub struct DeviceData {
    /// Device kind.
    pub kind: DeviceKind,
    /// Device product identifier, better known as product ID.
    #[serde(rename = "product ID")]
    pub product_id: Option<alloc::borrow::Cow<'static, str>>,
    /// Device environment.
    pub environment: DeviceEnvironment,
    /// Device main route.
    #[serde(rename = "main route")]
    pub main_route: alloc::borrow::Cow<'static, str>,
    /// All device route configurations.
    pub route_configs: RouteConfigs,
}

#[cfg(feature = "alloc")]
impl DeviceData {
    /// Creates a [`DeviceData`].
    #[must_use]
    pub fn new(
        kind: DeviceKind,
        environment: DeviceEnvironment,
        main_route: impl Into<alloc::borrow::Cow<'static, str>>,
        route_configs: RouteConfigs,
    ) -> Self {
        Self {
            kind,
            product_id: None,
            environment,
            main_route: main_route.into(),
            route_configs,
        }
    }

    /// Adds a device product identifier, better known as product ID.
    #[must_use]
    #[inline]
    pub fn product_id(mut self, product_id: impl Into<alloc::borrow::Cow<'static, str>>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }
}

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use crate::hazards::{Hazard, Hazards};
    use crate::parameters::Parameters;
    use crate::route::{Route, RouteConfigs};
    use crate::{deserialize, serialize};

    use super::{DeviceData, DeviceEnvironment, DeviceKind};

    const PRODUCT_ID: &str = "00000018283";

    fn create_route_configs() -> RouteConfigs {
        let light_on_route = Route::put("/on")
            .description("Turn light on.")
            .with_hazard(Hazard::ElectricEnergyConsumption);

        let light_off_route = Route::put("/off")
            .description("Turn light off.")
            .with_hazard(Hazard::LogEnergyConsumption);

        let toggle_route = Route::get("/toggle")
            .description("Toggle a light.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::ElectricEnergyConsumption),
            )
            .with_parameters(Parameters::new().rangeu64("brightness", (0, 20, 1)));

        RouteConfigs::new()
            .insert(light_on_route.serialize_data())
            .insert(light_off_route.serialize_data())
            .insert(toggle_route.serialize_data())
    }

    fn expected_device_data(product_id: Option<Cow<'static, str>>) -> DeviceData {
        DeviceData {
            kind: DeviceKind::Light,
            product_id,
            environment: DeviceEnvironment::Os,
            main_route: "light/".into(),
            route_configs: create_route_configs(),
        }
    }

    #[test]
    fn test_device_data() {
        let route_configs = create_route_configs();

        assert_eq!(
            deserialize::<DeviceData>(serialize(DeviceData::new(
                DeviceKind::Light,
                DeviceEnvironment::Os,
                "light/",
                route_configs.clone(),
            ))),
            expected_device_data(None)
        );

        assert_eq!(
            deserialize::<DeviceData>(serialize(
                DeviceData::new(
                    DeviceKind::Light,
                    DeviceEnvironment::Os,
                    "light/",
                    route_configs,
                )
                .product_id(PRODUCT_ID)
            )),
            expected_device_data(Some(PRODUCT_ID.into()))
        );
    }
}
