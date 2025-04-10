use serde::Serialize;

use crate::energy::EnergyClass;

/// Timespan for a cost computation.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum CostTimespan {
    /// Week
    Week,
    /// Month
    Month,
    /// Year
    Year,
}

impl CostTimespan {
    const fn name(self) -> &'static str {
        match self {
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

impl core::fmt::Display for CostTimespan {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.name().fmt(f)
    }
}

/// A device cost in terms of expenses/savings.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Cost {
    /// Amount of money in USD currency.
    #[serde(rename = "usd")]
    pub usd_currency: i32,
    /// Considered timespan.
    pub timespan: CostTimespan,
}

impl core::fmt::Display for Cost {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "The device {} {} USD in a {} timespan",
            if self.usd_currency < 0 {
                "saves"
            } else {
                "spends"
            },
            self.usd_currency.abs(),
            self.timespan
        )
    }
}

impl Cost {
    /// Creates a [`Cost`] instance.
    #[must_use]
    pub const fn new(usd_currency: i32, timespan: CostTimespan) -> Self {
        Self {
            usd_currency,
            timespan,
        }
    }
}

/// A collection of [`Cost`]s.
#[cfg(feature = "alloc")]
pub type Costs = crate::collections::OutputSet<Cost>;

/// Return on investments (ROI).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Roi {
    /// Years timespan to calculate the ROI.
    pub years: u8,
    /// Energy class.
    #[serde(rename = "energy-class")]
    pub energy_class: EnergyClass,
}

impl core::fmt::Display for Roi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "The device has a Return on Investments (Roi) for the `{}` \
            energy efficiency class over a timespan of {} {}",
            self.energy_class,
            self.years,
            if self.years > 1 { "years" } else { "year" },
        )
    }
}

impl Roi {
    /// Creates a [`Roi`] instance.
    ///
    /// If the `years` parameter is equal to **0**, the value of **1**
    /// is automatically being set.
    /// If the `years` parameter is greater than **30**, the value of **30** is
    /// automatically being set.
    #[must_use]
    pub const fn new(years: u8, energy_class: EnergyClass) -> Self {
        let years = match years {
            0 => 1,
            30.. => 30,
            _ => years,
        };
        Self {
            years,
            energy_class,
        }
    }
}

/// A collection of [`Roi`]s.
#[cfg(feature = "alloc")]
pub type Rois = crate::collections::OutputSet<Roi>;

/// Economy data for a device.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Clone, Serialize, serde::Deserialize)]
pub struct Economy {
    /// Costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<Costs>,
    /// Return on investments (ROI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roi: Option<Rois>,
}

#[cfg(feature = "alloc")]
impl Economy {
    /// Creates an empty [`Economy`] instance.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            costs: None,
            roi: None,
        }
    }

    /// Creates a new [`Economy`] instance initialized with
    /// [`Costs`] data.
    #[must_use]
    pub const fn init_with_costs(costs: Costs) -> Self {
        Self {
            costs: Some(costs),
            roi: None,
        }
    }

    /// Creates a new [`Economy`] instance initialized with
    /// [`Rois`] data.
    #[must_use]
    pub const fn init_with_roi(roi: Rois) -> Self {
        Self {
            costs: None,
            roi: Some(roi),
        }
    }

    /// Adds [`Costs`] data.
    #[must_use]
    #[inline]
    pub fn costs(mut self, costs: Costs) -> Self {
        self.costs = Some(costs);
        self
    }

    /// Adds [`Rois`] data.
    #[must_use]
    #[inline]
    pub fn roi(mut self, roi: Rois) -> Self {
        self.roi = Some(roi);
        self
    }

    /// Checks whether [`Economy`] is **completely** empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.costs.is_none() && self.roi.is_none()
    }
}
