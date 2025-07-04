use serde::{Deserialize, Serialize};

#[cfg(feature = "alloc")]
use crate::economy::Economy;
#[cfg(feature = "alloc")]
use crate::energy::Energy;
#[cfg(feature = "alloc")]
use crate::route::RouteConfigs;

/// A device kind.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceKind {
    /// Unknown.
    Unknown,
    /// Light.
    Light,
    /// Camera.
    Camera,
}

impl DeviceKind {
    const fn description(self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Light => "Light",
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DeviceEnvironment {
    /// Operating system.
    Os,
    /// Esp32.
    Esp32,
}

/// Device information.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData {
    /// Device kind.
    pub kind: DeviceKind,
    /// Device environment.
    pub environment: DeviceEnvironment,
    /// Device main route.
    #[serde(rename = "main route")]
    pub main_route: alloc::borrow::Cow<'static, str>,
    /// All device route configurations.
    pub route_configs: RouteConfigs,
    /// Wi-Fi MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<[u8; 6]>,
    /// Ethernet MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet_mac: Option<[u8; 6]>,
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
        wifi_mac: Option<[u8; 6]>,
        ethernet_mac: Option<[u8; 6]>,
    ) -> Self {
        Self {
            kind,
            environment,
            main_route: main_route.into(),
            route_configs,
            wifi_mac,
            ethernet_mac,
        }
    }
}
