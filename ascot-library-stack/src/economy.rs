use ascot_library::economy::{Cost, Roi};
use serde::{Deserialize, Serialize};

use crate::utils::collections::OutputCollection;

/// A collection of [`Cost`]s.
pub type Costs<const N: usize> = OutputCollection<Cost, N>;

/// A collection of [`Roi`]s.
pub type Rois<const N: usize> = OutputCollection<Roi, N>;

/// Economy data for a device.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Economy<const C: usize, const R: usize> {
    /// Costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<Costs<C>>,
    /// Return on investments (ROI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roi: Option<Rois<R>>,
}

impl<const C: usize, const R: usize> Economy<C, R> {
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
    pub const fn init_with_costs(costs: Costs<C>) -> Self {
        Self {
            costs: Some(costs),
            roi: None,
        }
    }

    /// Creates a new [`Economy`] instance initialized with
    /// [`Rois`] data.
    #[must_use]
    pub const fn init_with_roi(roi: Rois<R>) -> Self {
        Self {
            costs: None,
            roi: Some(roi),
        }
    }

    /// Adds [`Costs`] data.
    #[must_use]
    #[inline]
    pub fn costs<const C2: usize>(self, costs: Costs<C2>) -> Economy<C2, R> {
        Economy::<C2, R> {
            costs: Some(costs),
            roi: self.roi,
        }
    }

    /// Adds [`Rois`] data.
    #[must_use]
    #[inline]
    pub fn rois<const R2: usize>(self, roi: Rois<R2>) -> Economy<C, R2> {
        Economy::<C, R2> {
            costs: self.costs,
            roi: Some(roi),
        }
    }

    /// Checks whether [`Economy`] is **completely** empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.costs.is_none() && self.roi.is_none()
    }
}
