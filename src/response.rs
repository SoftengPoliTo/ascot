#[cfg(feature = "alloc")]
use alloc::string::String;

use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "alloc")]
use crate::actions::ActionError;

#[cfg(feature = "alloc")]
use crate::device::DeviceInfo;

/// Action response kinds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum ResponseKind {
    /// A short message to notify a receiver that an action has terminated
    /// correctly.
    #[default]
    Ok,
    /// Serial data (i.e. JSON).
    ///
    /// This response provides more detailed information about an action.
    Serial,
    /// Informative data to describe a device (i.e. JSON).
    ///
    /// This response provides economy and energy information of a device.
    Info,
    /// Stream of data expressed as a sequence of bytes.
    Stream,
}

impl core::fmt::Display for ResponseKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Ok => "Ok",
            Self::Serial => "Serial",
            Self::Info => "Info",
            Self::Stream => "Stream",
        }
        .fmt(f)
    }
}

/// An `Ok` response sends a boolean to notify a receiver that a device action
/// has terminated correctly.
#[derive(Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OkResponse {
    action_terminated_correctly: bool,
}

impl OkResponse {
    /// Creates an [`OkResponse`].
    #[must_use]
    #[inline]
    pub fn ok() -> Self {
        Self {
            action_terminated_correctly: true,
        }
    }
}

/// Serial response.
///
/// This response provides more detailed information about an action.
#[derive(Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct SerialResponse<T: DeserializeOwned> {
    #[serde(flatten)]
    data: T,
}

impl<T: Serialize + DeserializeOwned> SerialResponse<T> {
    /// Creates a [`SerialResponse`].
    #[must_use]
    pub const fn new(data: T) -> Self {
        Self { data }
    }
}

/// Informative response.
///
/// This response provides economy and energy information of a device.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
pub struct InfoResponse {
    #[serde(flatten)]
    data: DeviceInfo,
}

#[cfg(feature = "alloc")]
impl InfoResponse {
    /// Creates a [`InfoResponse`].
    #[must_use]
    pub const fn new(data: DeviceInfo) -> Self {
        Self { data }
    }
}

/// A response containing structured information about an error occurred during
/// the execution of an action.
///
/// It describes the kind of error, the cause, and optional information.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    /// Action error type.
    pub error: ActionError,
    /// Error description.
    pub description: String,
    /// Information about an error.
    pub info: Option<String>,
}

#[cfg(feature = "alloc")]
impl ErrorResponse {
    /// Creates an [`ErrorResponse`] with a specific [`ActionError`] and
    /// a description.
    #[must_use]
    #[inline]
    pub fn with_description(error: ActionError, description: &str) -> Self {
        Self {
            error,
            description: String::from(description),
            info: None,
        }
    }

    /// Creates an [`ErrorResponse`] with a specific [`ActionError`], an
    /// error description, and additional information about the error.
    #[must_use]
    #[inline]
    pub fn with_description_error(error: ActionError, description: &str, info: &str) -> Self {
        Self {
            error,
            description: String::from(description),
            info: Some(String::from(info)),
        }
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description.
    #[must_use]
    #[inline]
    pub fn invalid_data(description: &str) -> Self {
        Self::with_description(ActionError::InvalidData, description)
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description and
    /// additional information about the error.
    #[must_use]
    #[inline]
    pub fn invalid_data_with_error(description: &str, info: &str) -> Self {
        Self::with_description_error(ActionError::InvalidData, description, info)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description.
    #[must_use]
    #[inline]
    pub fn internal(description: &str) -> Self {
        Self::with_description(ActionError::Internal, description)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description
    /// and additional information about the error.
    #[must_use]
    #[inline]
    pub fn internal_with_error(description: &str, info: &str) -> Self {
        Self::with_description_error(ActionError::Internal, description, info)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "alloc")]
    use super::{ActionError, DeviceInfo, ErrorResponse, InfoResponse, String};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Serial {
        value: u32,
    }

    #[cfg(feature = "deserialize")]
    #[test]
    fn test_ok_response() {
        assert_eq!(
            crate::deserialize::<super::OkResponse>(crate::serialize(super::OkResponse::ok())),
            super::OkResponse {
                action_terminated_correctly: true,
            }
        );
    }

    #[cfg(feature = "deserialize")]
    #[test]
    fn test_serial_response() {
        assert_eq!(
            crate::deserialize::<Serial>(crate::serialize(super::SerialResponse::new(Serial {
                value: 42
            }))),
            Serial { value: 42 },
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_info_response() {
        let energy = crate::energy::Energy::init_with_water_use_efficiency(
            crate::energy::WaterUseEfficiency::init_with_gpp(42.0),
        );

        assert_eq!(
            crate::deserialize::<DeviceInfo>(crate::serialize(InfoResponse::new(
                DeviceInfo::empty().add_energy(energy)
            ))),
            DeviceInfo {
                energy: crate::energy::Energy {
                    energy_efficiencies: None,
                    carbon_footprints: None,
                    water_use_efficiency: Some(crate::energy::WaterUseEfficiency {
                        gpp: Some(42.0),
                        penman_monteith_equation: None,
                        wer: None,
                    }),
                },
                economy: crate::economy::Economy::empty(),
            }
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_error_response() {
        let error = ErrorResponse::with_description(
            ActionError::InvalidData,
            "Invalid data error description",
        );

        assert_eq!(
            crate::deserialize::<ErrorResponse>(crate::serialize(error)),
            ErrorResponse {
                error: ActionError::InvalidData,
                description: String::from("Invalid data error description"),
                info: None,
            }
        );
    }
}
