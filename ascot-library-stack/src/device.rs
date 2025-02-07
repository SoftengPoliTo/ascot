use serde::{Deserialize, Serialize};

use crate::economy::Economy;
use crate::energy::Energy;
use crate::route::RouteConfigs;

pub use ascot_library::device::{DeviceEnvironment, DeviceKind};

/// Device information.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
            environment,
            main_route,
            route_configs,
        }
    }
}
