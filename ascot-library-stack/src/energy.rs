use ascot_library::energy::{CarbonFootprint, EnergyEfficiency, WaterUseEfficiency};
use serde::{Deserialize, Serialize};

use crate::utils::collections::OutputCollection;

/// A collection of [`EnergyEfficiency`]s.
pub type EnergyEfficiencies<const E: usize> = OutputCollection<EnergyEfficiency, E>;

/// A collection of [`CarbonFootprints`]s.
pub type CarbonFootprints<const CF: usize> = OutputCollection<CarbonFootprint, CF>;

/// Energy information of a device.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Energy<const E: usize, const CF: usize> {
    /// Energy efficiencies.
    #[serde(rename = "energy-efficiencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_efficiencies: Option<EnergyEfficiencies<E>>,
    /// Carbon footprints.
    #[serde(rename = "carbon-footprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbon_footprints: Option<CarbonFootprints<CF>>,
    /// Water-Use efficiency.
    #[serde(rename = "water-use-efficiency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_use_efficiency: Option<WaterUseEfficiency>,
}

impl<const E: usize, const CF: usize> Energy<E, CF> {
    /// Creates an empty [`Energy`] instance.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            energy_efficiencies: None,
            carbon_footprints: None,
            water_use_efficiency: None,
        }
    }

    /// Creates a new [`Energy`] instance initialized with
    /// [`EnergyEfficiencies`] data.
    #[must_use]
    pub const fn init_with_energy_efficiencies(energy_efficiencies: EnergyEfficiencies<E>) -> Self {
        Self {
            energy_efficiencies: Some(energy_efficiencies),
            carbon_footprints: None,
            water_use_efficiency: None,
        }
    }

    /// Creates a new [`Energy`] instance initialized with
    /// [`CarbonFootprints`] data.
    #[must_use]
    pub const fn init_with_carbon_footprints(carbon_footprints: CarbonFootprints<N>) -> Self {
        Self {
            energy_efficiencies: None,
            carbon_footprints: Some(carbon_footprints),
            water_use_efficiency: None,
        }
    }

    /// Creates a new [`Energy`] instance initialized with
    /// [`WaterUseEfficiency`] data.
    #[must_use]
    pub const fn init_with_water_use_efficiency(water_use_efficiency: WaterUseEfficiency) -> Self {
        Self {
            energy_efficiencies: None,
            carbon_footprints: None,
            water_use_efficiency: Some(water_use_efficiency),
        }
    }

    /// Adds [`EnergyEfficiencies`] data.
    #[must_use]
    #[inline]
    pub fn energy_efficiencies(mut self, energy_efficiencies: EnergyEfficiencies<N>) -> Self {
        self.energy_efficiencies = Some(energy_efficiencies);
        self
    }

    /// Adds [`CarbonFootprints`] data.
    #[must_use]
    #[inline]
    pub fn carbon_footprints(mut self, carbon_footprints: CarbonFootprints<N>) -> Self {
        self.carbon_footprints = Some(carbon_footprints);
        self
    }

    /// Adds [`WaterUseEfficiency`] data.
    #[must_use]
    pub const fn water_use_efficiency(mut self, water_use_efficiency: WaterUseEfficiency) -> Self {
        self.water_use_efficiency = Some(water_use_efficiency);
        self
    }

    /// Checks whether [`Energy`] is **completely** empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.energy_efficiencies.is_none()
            && self.carbon_footprints.is_none()
            && self.water_use_efficiency.is_none()
    }
}
