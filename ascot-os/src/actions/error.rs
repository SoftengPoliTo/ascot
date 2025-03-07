use ascot::actions::ActionError;
use ascot::response::ErrorResponse as AscotErrorResponse;

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// A response containing structured information about an error occurred during
/// the execution of an action.
///
/// It describes the kind of error, the cause, and optional information.
pub struct ErrorResponse(AscotErrorResponse);

impl ErrorResponse {
    /// Creates an [`ErrorResponse`] with a specific [`ActionError`] and
    /// a description.
    ///
    /// If an error occurs, an empty description is returned.
    #[must_use]
    #[inline]
    pub fn with_description(error: ActionError, description: &str) -> Self {
        Self(AscotErrorResponse::with_description(error, description))
    }

    /// Creates an [`ErrorResponse`] with a specific [`ActionError`], an
    /// error description, and additional information about the error.
    ///
    /// If this method fails for some internal reasons, empty description and
    /// information are returned.
    #[must_use]
    #[inline]
    pub fn with_description_error(
        error: ActionError,
        description: &str,
        info: impl std::error::Error,
    ) -> Self {
        Self(AscotErrorResponse::with_description_error(
            error,
            description,
            &info.to_string(),
        ))
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description.
    ///
    /// If this method fails for some internal reasons, an empty description
    /// is returned.
    #[must_use]
    #[inline]
    pub fn invalid_data(description: &str) -> Self {
        Self::with_description(ActionError::InvalidData, description)
    }

    /// Creates an [`ErrorResponse`] for invalid data with a description and
    /// additional information about the error.
    ///
    /// If this method fails for some internal reasons, empty description and
    /// information are returned.
    #[must_use]
    #[inline]
    pub fn invalid_data_with_error(description: &str, error: impl std::error::Error) -> Self {
        Self::with_description_error(ActionError::InvalidData, description, error)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description.
    ///
    /// If this method fails for some internal reasons, an empty description
    /// is returned.
    #[must_use]
    #[inline]
    pub fn internal(description: &str) -> Self {
        Self::with_description(ActionError::Internal, description)
    }

    /// Creates an [`ErrorResponse`] for an internal error with a description and
    /// additional information about the error.
    ///
    /// If this method fails for some internal reasons, empty description and
    /// information are returned.
    #[must_use]
    #[inline]
    pub fn internal_with_error(description: &str, error: impl std::error::Error) -> Self {
        Self::with_description_error(ActionError::Internal, description, error)
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.0)).into_response()
    }
}
