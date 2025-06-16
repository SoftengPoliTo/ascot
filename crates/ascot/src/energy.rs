use serde::{Deserialize, Serialize};

/// Energy efficiency class.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum EnergyClass {
    /// A+++
    #[serde(rename = "A+++")]
    APlusPlusPlus,
    /// A++
    #[serde(rename = "A++")]
    APlusPlus,
    /// A+
    #[serde(rename = "A+")]
    APlus,
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
}

impl EnergyClass {
    const fn name(self) -> &'static str {
        match self {
            Self::APlusPlusPlus => "A+++",
            Self::APlusPlus => "A++",
            Self::APlus => "A+",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
        }
    }
}

impl core::fmt::Display for EnergyClass {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.name().fmt(f)
    }
}

const fn decimal_percentage(percentage: i8) -> f64 {
    percentage as f64 / 100.
}

/// Energy efficiency.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct EnergyEfficiency {
    /// Energy efficiency savings or consumes for the relevant [`EnergyClass`].
    pub percentage: i8,
    /// Energy class.
    #[serde(rename = "energy-class")]
    pub energy_class: EnergyClass,
}

impl core::fmt::Display for EnergyEfficiency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "The device {} a {}% of energy for the \"{}\" efficiency class",
            if self.percentage < 0 {
                "saves"
            } else {
                "consumes"
            },
            self.percentage.abs(),
            self.energy_class
        )
    }
}

impl EnergyEfficiency {
    /// Creates an [`EnergyEfficiency`] instance.
    ///
    /// If the `percentage` parameter is lower than -100, the value of -100
    /// is automatically being set.
    /// If the `percentage` parameter is greater than 100, the value of 100 is
    /// automatically being set.
    #[must_use]
    pub const fn new(percentage: i8, energy_class: EnergyClass) -> Self {
        let percentage = match percentage {
            100.. => 100,
            ..=-100 => -100,
            _ => percentage,
        };
        Self {
            percentage,
            energy_class,
        }
    }

    /// Returns the [`EnergyEfficiency`] percentage as decimal value.
    #[must_use]
    pub const fn decimal_percentage(&self) -> f64 {
        decimal_percentage(self.percentage)
    }
}

/// A collection of [`EnergyEfficiency`]s.
#[cfg(feature = "alloc")]
pub type EnergyEfficiencies = crate::collections::OutputSet<EnergyEfficiency>;

/// Carbon footprint.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct CarbonFootprint {
    /// The percentage of greenhouse gases added or removed from the atmosphere
    /// for the relevant [`EnergyClass`].
    pub percentage: i8,
    /// Energy class.
    #[serde(rename = "energy-class")]
    pub energy_class: EnergyClass,
}

impl core::fmt::Display for CarbonFootprint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "The device {} the atmosphere a {}% of greenhouse gases for the \"{}\" efficiency class",
            if self.percentage < 0 {
                "removes from"
            } else {
                "adds to"
            },
            self.percentage.abs(),
            self.energy_class
        )
    }
}

impl CarbonFootprint {
    /// Creates a [`CarbonFootprint`] instance.
    ///
    /// If the `percentage` parameter is lower than -100, the value of -100
    /// is automatically being set.
    /// If the `percentage` parameter is greater than 100, the value of 100 is
    /// automatically being set.
    #[must_use]
    pub const fn new(percentage: i8, energy_class: EnergyClass) -> Self {
        let percentage = match percentage {
            100.. => 100,
            ..=-100 => -100,
            _ => percentage,
        };
        Self {
            percentage,
            energy_class,
        }
    }

    /// Returns the [`CarbonFootprint`] percentage as decimal value.
    #[must_use]
    pub const fn decimal_percentage(&self) -> f64 {
        decimal_percentage(self.percentage)
    }
}

/// A collection of [`CarbonFootprints`]s.
#[cfg(feature = "alloc")]
pub type CarbonFootprints = crate::collections::OutputSet<CarbonFootprint>;

/// Water-Use efficiency data.
///
/// Metrics taken from:
/// <https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2019.00103/full>
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct WaterUseEfficiency {
    /// Gross Primary Productivity (GPP).
    ///
    /// Article: <https://www.sciencedirect.com/science/article/abs/pii/S0168192313002141>
    #[serde(rename = "gross-primary-productivity")]
    pub gpp: Option<f64>,
    /// Penman–Monteith Equation.
    ///
    /// Article: <https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2019.00103/full#B7>
    #[serde(rename = "penman-monteith-equation")]
    pub penman_monteith_equation: Option<f64>,
    /// Water Equivalent Ratio (WER).
    ///
    /// Article: <https://www.sciencedirect.com/science/article/abs/pii/S0378377416303924>
    #[serde(rename = "water-equivalent-ratio")]
    pub wer: Option<f64>,
}

impl WaterUseEfficiency {
    /// Creates a new [`WaterUseEfficiency`] instance initialized with
    /// `GPP` metric.
    #[must_use]
    pub const fn init_with_gpp(gpp: f64) -> Self {
        Self {
            gpp: Some(gpp),
            penman_monteith_equation: None,
            wer: None,
        }
    }

    /// Creates a new [`WaterUseEfficiency`] instance initialized with
    /// `Penman-Monteith Equation` metric.
    #[must_use]
    pub const fn init_with_penman_monteith_equation(penman_monteith_equation: f64) -> Self {
        Self {
            gpp: None,
            penman_monteith_equation: Some(penman_monteith_equation),
            wer: None,
        }
    }

    /// Creates a new [`WaterUseEfficiency`] instance initialized with
    /// `Water Equivalent Ratio (WER)` metric.
    #[must_use]
    pub const fn init_with_wer(wer: f64) -> Self {
        Self {
            gpp: None,
            penman_monteith_equation: None,
            wer: Some(wer),
        }
    }

    /// Adds `GPP` metric.
    #[must_use]
    pub const fn gpp(mut self, gpp: f64) -> Self {
        self.gpp = Some(gpp);
        self
    }

    /// Adds `Penman-Monteith Equation` metric.
    #[must_use]
    pub const fn penman_monteith_equation(mut self, penman_monteith_equation: f64) -> Self {
        self.penman_monteith_equation = Some(penman_monteith_equation);
        self
    }

    /// Adds `Water Equivalent Ratio (WER)` metric.
    #[must_use]
    pub const fn wer(mut self, wer: f64) -> Self {
        self.wer = Some(wer);
        self
    }
}

/// Energy information of a device.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Energy {
    /// Energy efficiencies.
    #[serde(rename = "energy-efficiencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_efficiencies: Option<EnergyEfficiencies>,
    /// Carbon footprints.
    #[serde(rename = "carbon-footprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbon_footprints: Option<CarbonFootprints>,
    /// Water-Use efficiency.
    #[serde(rename = "water-use-efficiency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_use_efficiency: Option<WaterUseEfficiency>,
}

#[cfg(feature = "alloc")]
impl Energy {
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
    pub const fn init_with_energy_efficiencies(energy_efficiencies: EnergyEfficiencies) -> Self {
        Self {
            energy_efficiencies: Some(energy_efficiencies),
            carbon_footprints: None,
            water_use_efficiency: None,
        }
    }

    /// Creates a new [`Energy`] instance initialized with
    /// [`CarbonFootprints`] data.
    #[must_use]
    pub const fn init_with_carbon_footprints(carbon_footprints: CarbonFootprints) -> Self {
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
    pub fn energy_efficiencies(mut self, energy_efficiencies: EnergyEfficiencies) -> Self {
        self.energy_efficiencies = Some(energy_efficiencies);
        self
    }

    /// Adds [`CarbonFootprints`] data.
    #[must_use]
    #[inline]
    pub fn carbon_footprints(mut self, carbon_footprints: CarbonFootprints) -> Self {
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use super::Energy;
    #[cfg(feature = "alloc")]
    use crate::collections::OutputSet;

    use crate::{deserialize, serialize};

    use super::{CarbonFootprint, EnergyClass, EnergyEfficiency, WaterUseEfficiency};

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn test_energy_class() {
        for energy_class in &[
            EnergyClass::APlusPlusPlus,
            EnergyClass::APlusPlus,
            EnergyClass::APlus,
            EnergyClass::A,
            EnergyClass::B,
            EnergyClass::C,
            EnergyClass::D,
            EnergyClass::E,
            EnergyClass::F,
            EnergyClass::G,
        ] {
            assert_eq!(
                deserialize::<EnergyClass>(serialize(energy_class)),
                *energy_class
            );
        }
    }

    #[test]
    fn test_energy_efficiency_serde() {
        let energy_efficiency = EnergyEfficiency::new(100, EnergyClass::A);

        assert_eq!(
            deserialize::<EnergyEfficiency>(serialize(energy_efficiency)),
            energy_efficiency
        );
    }

    #[test]
    fn test_energy_efficiency_clamping() {
        assert_eq!(EnergyEfficiency::new(127, EnergyClass::A).percentage, 100);
        assert_eq!(EnergyEfficiency::new(-128, EnergyClass::B).percentage, -100);
        assert_eq!(EnergyEfficiency::new(50, EnergyClass::C).percentage, 50);
    }

    #[test]
    fn test_energy_efficiency_decimal_percentage() {
        assert_float_eq(
            EnergyEfficiency::new(-50, EnergyClass::A).decimal_percentage(),
            -0.5,
        );
        assert_float_eq(
            EnergyEfficiency::new(50, EnergyClass::B).decimal_percentage(),
            0.5,
        );
    }

    #[test]
    fn test_carbon_footprint_serde() {
        let carbon_footprint = CarbonFootprint::new(100, EnergyClass::A);

        assert_eq!(
            deserialize::<CarbonFootprint>(serialize(carbon_footprint)),
            carbon_footprint
        );
    }

    #[test]
    fn test_carbon_footprint_clamping() {
        assert_eq!(CarbonFootprint::new(127, EnergyClass::A).percentage, 100);
        assert_eq!(CarbonFootprint::new(-128, EnergyClass::B).percentage, -100);
        assert_eq!(CarbonFootprint::new(50, EnergyClass::C).percentage, 50);
    }

    #[test]
    fn test_carbon_footprint_decimal_percentage() {
        assert_float_eq(
            CarbonFootprint::new(-50, EnergyClass::A).decimal_percentage(),
            -0.5,
        );
        assert_float_eq(
            CarbonFootprint::new(50, EnergyClass::B).decimal_percentage(),
            0.5,
        );
    }

    #[test]
    fn test_water_use_efficiency_serde() {
        let water_use_efficiency = WaterUseEfficiency::init_with_gpp(2.5)
            .penman_monteith_equation(3.2)
            .wer(1.1);

        assert_eq!(
            deserialize::<WaterUseEfficiency>(serialize(water_use_efficiency)),
            water_use_efficiency
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_energy() {
        let mut energy = Energy::empty();

        let energy_efficiencies = OutputSet::init(EnergyEfficiency::new(-50, EnergyClass::A))
            .insert(EnergyEfficiency::new(50, EnergyClass::B));

        let carbon_footprints = OutputSet::init(CarbonFootprint::new(-50, EnergyClass::A))
            .insert(CarbonFootprint::new(50, EnergyClass::B));

        let water_use_efficiency = WaterUseEfficiency::init_with_gpp(2.5)
            .penman_monteith_equation(3.2)
            .wer(1.1);

        assert!(energy.is_empty());

        energy = energy
            .energy_efficiencies(energy_efficiencies)
            .carbon_footprints(carbon_footprints)
            .water_use_efficiency(water_use_efficiency);

        assert_eq!(deserialize::<Energy>(serialize(&energy)), energy);
    }
}
