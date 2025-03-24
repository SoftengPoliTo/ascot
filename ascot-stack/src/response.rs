use ascot::actions::ActionError;

use serde::Serialize;

use crate::device::DeviceInfo;
use crate::string::String;

pub use ascot::response::{OkResponse, ResponseKind, SerialResponse};

/// Informative response.
///
/// This response provides economy and energy information of a device.
#[derive(Debug, PartialEq, Serialize)]
pub struct InfoResponse<const C: usize, const R: usize, const E: usize, const CF: usize> {
    #[serde(flatten)]
    data: DeviceInfo<C, R, E, CF>,
}

impl<const C: usize, const R: usize, const E: usize, const CF: usize> InfoResponse<C, R, E, CF> {
    /// Creates a [`InfoResponse`].
    #[must_use]
    pub const fn new(data: DeviceInfo<C, R, E, CF>) -> Self {
        Self { data }
    }
}

/// A response containing structured information about an error occurred during
/// the execution of an action.
///
/// It describes the kind of error, the cause, and optional information.
#[derive(Debug, PartialEq, Serialize)]
pub struct ErrorResponse<const N: usize> {
    /// Action error type.
    pub error: ActionError,
    /// Error description.
    pub description: String<N>,
    /// Information about an error.
    pub info: Option<String<N>>,
}

impl<const N: usize> ErrorResponse<N> {
    /// Creates an [`ErrorResponse`] with a specific [`ActionError`] and
    /// a description.
    ///
    /// If the `description` field is returned empty, it means that an error
    /// occurred while creating its value.
    #[must_use]
    #[inline]
    pub fn with_description(error: ActionError, description: &str) -> Self {
        Self {
            error,
            description: String::infallible(description),
            info: None,
        }
    }

    /// Creates an [`ErrorResponse`] with a specific [`ActionError`], an
    /// error description, and additional information about the error.
    ///
    /// If `description` and `information` fields are returned empty, it means
    /// that an error occurred while creating their values.
    #[must_use]
    #[inline]
    pub fn with_description_error(error: ActionError, description: &str, info: &str) -> Self {
        Self {
            error,
            description: String::infallible(description),
            info: Some(String::infallible(info)),
        }
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description.
    ///
    /// If the `description` field is returned empty, it means that an error
    /// occurred while creating its value.
    #[must_use]
    #[inline]
    pub fn invalid_data(description: &str) -> Self {
        Self::with_description(ActionError::InvalidData, description)
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description and
    /// additional information about the error.
    ///
    /// If `description` and `information` fields are returned empty, it means
    /// that an error occurred while creating their values.
    #[must_use]
    #[inline]
    pub fn invalid_data_with_error(description: &str, info: &str) -> Self {
        Self::with_description_error(ActionError::InvalidData, description, info)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description.
    ///
    /// If the `description` field is returned empty, it means that an error
    /// occurred while creating its value.
    #[must_use]
    #[inline]
    pub fn internal(description: &str) -> Self {
        Self::with_description(ActionError::Internal, description)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description
    /// and additional information about the error.
    ///
    /// If `description` and `information` fields are returned empty, it means
    /// that an error occurred while creating their values.
    #[must_use]
    #[inline]
    pub fn internal_with_error(description: &str, info: &str) -> Self {
        Self::with_description_error(ActionError::Internal, description, info)
    }
}

#[cfg(test)]
mod tests {
    use ascot::actions::ActionError;
    use serde_json::json;

    use crate::serialize;

    use super::ErrorResponse;

    const STRING_SIZE: usize = 32;

    #[test]
    fn test_error_response() {
        let error = ErrorResponse::<STRING_SIZE>::with_description(
            ActionError::InvalidData,
            "Invalid data error description",
        );

        assert_eq!(
            serialize(error),
            json!({
                "description": "Invalid data error description",
                "error": "Invalid Data",
                "info": null
            })
        );
    }
}
