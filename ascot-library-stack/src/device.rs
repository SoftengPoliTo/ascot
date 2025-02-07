use ascot_library::device::{DeviceEnvironment, DeviceKind};

use serde::{Deserialize, Serialize};

use crate::economy::Economy;
use crate::energy::Energy;
use crate::route::RouteConfigs;

/// Device information.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeviceInfo<const N: usize> {
    /// Energy information.
    #[serde(skip_serializing_if = "Energy::is_empty")]
    #[serde(default = "Energy::empty")]
    pub energy: Energy<N>,
    /// Economy information.
    #[serde(skip_serializing_if = "Economy::is_empty")]
    #[serde(default = "Economy::empty")]
    pub economy: Economy<N>,
}

impl<const N: usize> DeviceInfo<N> {
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
    pub fn add_energy(mut self, energy: Energy<N>) -> Self {
        self.energy = energy;
        self
    }

    /// Adds [`Economy`] data.
    #[must_use]
    pub fn add_economy(mut self, economy: Economy<N>) -> Self {
        self.economy = economy;
        self
    }
}

/// Device data.
#[derive(Debug, Serialize)]
pub struct DeviceData<const N: usize> {
    /// Device kind.
    pub kind: DeviceKind,
    /// Device environment.
    pub environment: DeviceEnvironment,
    /// Device main route.
    #[serde(rename = "main route")]
    pub main_route: &'static str,
    /// All device route configurations.
    pub route_configs: RouteConfigs<N>,
}

impl<const N: usize> DeviceData<N> {
    /// Creates a [`DeviceData`].
    #[must_use]
    pub fn new(
        kind: DeviceKind,
        environment: DeviceEnvironment,
        main_route: &'static str,
        route_configs: RouteConfigs<N>,
    ) -> Self {
        Self {
            kind,
            environment,
            main_route,
            route_configs,
        }
    }
}
