use ascot_library::economy::{Cost, Roi};
use serde::{Deserialize, Serialize};

use crate::utils::collections::OutputCollection;

/// A collection of [`Cost`]s.
pub type Costs<const N: usize> = OutputCollection<Cost, N>;

/// A collection of [`Roi`]s.
pub type Rois<const N: usize> = OutputCollection<Roi, N>;

/// Economy data for a device.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Economy<const N: usize> {
    /// Costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<Costs<N>>,
    /// Return on investments (ROI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roi: Option<Rois<N>>,
}

impl<const N: usize> Economy<N> {
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
    pub const fn init_with_costs(costs: Costs<N>) -> Self {
        Self {
            costs: Some(costs),
            roi: None,
        }
    }

    /// Creates a new [`Economy`] instance initialized with
    /// [`Rois`] data.
    #[must_use]
    pub const fn init_with_roi(roi: Rois<N>) -> Self {
        Self {
            costs: None,
            roi: Some(roi),
        }
    }

    /// Adds [`Costs`] data.
    #[must_use]
    #[inline]
    pub fn costs(mut self, costs: Costs<N>) -> Self {
        self.costs = Some(costs);
        self
    }

    /// Adds [`Rois`] data.
    #[must_use]
    #[inline]
    pub fn roi(mut self, roi: Rois<N>) -> Self {
        self.roi = Some(roi);
        self
    }

    /// Checks whether [`Economy`] is **completely** empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.costs.is_none() && self.roi.is_none()
    }
}
