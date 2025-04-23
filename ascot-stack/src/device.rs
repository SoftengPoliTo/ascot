use serde::Serialize;

use crate::economy::Economy;
use crate::energy::Energy;
use crate::route::RouteConfigs;

pub use ascot::device::{DeviceEnvironment, DeviceKind};

/// Device information.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct DeviceInfo<const C: usize, const R: usize, const E: usize, const CF: usize> {
    /// Economy information.
    #[serde(skip_serializing_if = "Economy::is_empty")]
    #[serde(default = "Economy::empty")]
    pub economy: Economy<C, R>,
    /// Energy information.
    #[serde(skip_serializing_if = "Energy::is_empty")]
    #[serde(default = "Energy::empty")]
    pub energy: Energy<E, CF>,
}

impl DeviceInfo<2, 2, 2, 2> {
    /// Creates a [`DeviceInfo`].
    #[must_use]
    pub const fn empty() -> Self {
        DeviceInfo::<2, 2, 2, 2> {
            energy: Energy::<2, 2>::empty(),
            economy: Economy::<2, 2>::empty(),
        }
    }
}

impl<const C: usize, const R: usize, const E: usize, const CF: usize> DeviceInfo<C, R, E, CF> {
    /// Adds [`Energy`] data.
    #[must_use]
    #[inline]
    pub fn add_energy<const E2: usize, const CF2: usize>(
        self,
        energy: Energy<E2, CF2>,
    ) -> DeviceInfo<C, R, E2, CF2> {
        DeviceInfo::<C, R, E2, CF2> {
            energy,
            economy: self.economy,
        }
    }

    /// Adds [`Economy`] data.
    #[must_use]
    #[inline]
    pub fn add_economy<const C2: usize, const R2: usize>(
        self,
        economy: Economy<C2, R2>,
    ) -> DeviceInfo<C2, R2, E, CF> {
        DeviceInfo::<C2, R2, E, CF> {
            energy: self.energy,
            economy,
        }
    }
}

/// Device data.
#[derive(Debug, Serialize)]
pub struct DeviceData<const H: usize, const I: usize, const N: usize> {
    /// Device kind.
    pub kind: DeviceKind,
    /// Device product identifier, better known as product ID.
    #[serde(rename = "product ID")]
    pub product_id: Option<&'static str>,
    /// Device environment.
    pub environment: DeviceEnvironment,
    /// Device main route.
    #[serde(rename = "main route")]
    pub main_route: &'static str,
    /// All device route configurations.
    pub route_configs: RouteConfigs<H, I, N>,
}

impl<const H: usize, const I: usize, const N: usize> DeviceData<H, I, N> {
    /// Creates a [`DeviceData`].
    #[must_use]
    pub const fn new(
        kind: DeviceKind,
        environment: DeviceEnvironment,
        main_route: &'static str,
        route_configs: RouteConfigs<H, I, N>,
    ) -> Self {
        Self {
            kind,
            product_id: None,
            environment,
            main_route,
            route_configs,
        }
    }

    /// Adds a device product identifier, better known as product ID.
    #[must_use]
    pub const fn product_id(mut self, product_id: &'static str) -> Self {
        self.product_id = Some(product_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::hazards::{Hazard, Hazards};
    use crate::parameters::{ParameterKind, Parameters};
    use crate::route::{Route, RouteConfigs};
    use crate::serialize;

    use super::{DeviceData, DeviceEnvironment, DeviceKind};

    const PRODUCT_ID: &str = "00000018283";

    fn create_route_configs() -> RouteConfigs<2, 2, 4> {
        let light_on_route = Route::put("/on")
            .description("Turn light on.")
            .with_hazards(Hazards::one(Hazard::ElectricEnergyConsumption));

        let light_off_route = Route::put("/off")
            .description("Turn light off.")
            .with_hazards(Hazards::one(Hazard::LogEnergyConsumption));

        let toggle_route = Route::get("/toggle")
            .description("Toggle a light.")
            .with_hazards(Hazards::two((
                Hazard::FireHazard,
                Hazard::ElectricEnergyConsumption,
            )))
            .with_parameters(Parameters::one((
                "brightness",
                ParameterKind::rangeu64((0, 20, 1)),
            )));

        RouteConfigs::new()
            .insert(light_on_route.serialize_data())
            .insert(light_off_route.serialize_data())
            .insert(toggle_route.serialize_data())
    }

    fn expected_json(product_id: &Value) -> Value {
        json!(
            {
                "kind": "Light",
                "product ID": product_id,
                "environment": "Os",
                "main route": "light/",
                "route_configs":[
                    {
                        "name": "/on",
                        "description": "Turn light on.",
                        "REST kind": "Put",
                        "hazards": [
                            "ElectricEnergyConsumption"
                        ],
                        "response kind": "Ok"
                    },
                    {
                        "name": "/off",
                        "description": "Turn light off.",
                        "REST kind": "Put",
                        "hazards": [
                            "LogEnergyConsumption"
                        ],
                        "response kind": "Ok"
                    },
                    {
                        "name": "/toggle",
                        "description": "Toggle a light.",
                        "REST kind": "Get",
                        "hazards": [
                            "FireHazard",
                            "ElectricEnergyConsumption"
                        ],
                        "parameters": {
                            "brightness": {
                                "RangeU64": {
                                    "default":0,
                                    "max":20,
                                    "min":0,
                                    "step":1
                                }
                            }
                        },
                        "response kind": "Ok"
                    }
                ]
            }
        )
    }

    #[test]
    fn test_device_data() {
        assert_eq!(
            serialize(DeviceData::new(
                DeviceKind::Light,
                DeviceEnvironment::Os,
                "light/",
                create_route_configs(),
            )),
            expected_json(&json!(null))
        );

        assert_eq!(
            serialize(
                DeviceData::new(
                    DeviceKind::Light,
                    DeviceEnvironment::Os,
                    "light/",
                    create_route_configs(),
                )
                .product_id(PRODUCT_ID)
            ),
            expected_json(&json!(PRODUCT_ID))
        );
    }
}
