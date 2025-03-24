use serde::Serialize;

use crate::collections::create_set;

pub use ascot::energy::{CarbonFootprint, EnergyClass, EnergyEfficiency, WaterUseEfficiency};

create_set!(
    EnergyEfficiencies,
    EnergyEfficiency,
    energy_efficiency,
    energy_efficiencies
);

create_set!(
    CarbonFootprints,
    CarbonFootprint,
    carbon_footprint,
    carbon_footprints
);

/// Energy information of a device.
#[derive(Debug, PartialEq, Clone, Serialize)]
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
    pub const fn init_with_energy_efficiencies(
        energy_efficiencies: EnergyEfficiencies<E>,
    ) -> Energy<E, 2> {
        Energy::<E, 2> {
            energy_efficiencies: Some(energy_efficiencies),
            carbon_footprints: None,
            water_use_efficiency: None,
        }
    }

    /// Creates a new [`Energy`] instance initialized with
    /// [`CarbonFootprints`] data.
    #[must_use]
    pub const fn init_with_carbon_footprints(
        carbon_footprints: CarbonFootprints<CF>,
    ) -> Energy<2, CF> {
        Energy::<2, CF> {
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
    pub fn energy_efficiencies<const E2: usize>(
        self,
        energy_efficiencies: EnergyEfficiencies<E2>,
    ) -> Energy<E2, CF> {
        Energy::<E2, CF> {
            energy_efficiencies: Some(energy_efficiencies),
            carbon_footprints: self.carbon_footprints,
            water_use_efficiency: self.water_use_efficiency,
        }
    }

    /// Adds [`CarbonFootprints`] data.
    #[must_use]
    #[inline]
    pub fn carbon_footprints<const CF2: usize>(
        self,
        carbon_footprints: CarbonFootprints<CF2>,
    ) -> Energy<E, CF2> {
        Energy::<E, CF2> {
            energy_efficiencies: self.energy_efficiencies,
            carbon_footprints: Some(carbon_footprints),
            water_use_efficiency: self.water_use_efficiency,
        }
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
