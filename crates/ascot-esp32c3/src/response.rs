use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use ascot::actions::ActionError;
use ascot::response::{
    ErrorResponse as AscotErrorResponse, OkResponse as AscotOkResponse,
    SerialResponse as AscotSerialResponse,
};

use edge_http::io::server::Connection;
use edge_http::io::Error;

use embedded_io_async::{Read, Write};

use serde::{de::DeserializeOwned, Serialize};

/// Response headers.
pub struct Headers {
    status: u16,
    message: &'static str,
    content_type: &'static [(&'static str, &'static str)],
}

impl Headers {
    const fn ok() -> Self {
        Self {
            status: 200,
            message: "Ok",
            content_type: &[],
        }
    }

    const fn not_found() -> Self {
        Self {
            status: 404,
            message: "Not Found",
            content_type: &[],
        }
    }

    const fn not_allowed() -> Self {
        Self {
            status: 405,
            message: "Method Not Allowed",
            content_type: &[],
        }
    }

    const fn json() -> Self {
        Self {
            status: 200,
            message: "Ok",
            content_type: &[("Content-Type", "application/json")],
        }
    }

    const fn json_error() -> Self {
        Self {
            status: 500,
            message: "Error",
            content_type: &[("Content-Type", "application/json")],
        }
    }
}

/// A response body.
pub struct Body(Cow<'static, [u8]>);

impl Body {
    const fn empty() -> Self {
        Self(Cow::Borrowed(&[]))
    }

    const fn static_ref(v: &'static [u8]) -> Self {
        Self(Cow::Borrowed(v))
    }

    #[inline]
    fn owned(v: Vec<u8>) -> Self {
        Self(Cow::Owned(v))
    }
}

/// A server response.
pub struct Response {
    headers: Headers,
    body: Body,
}

impl Response {
    #[inline]
    pub(crate) async fn write<T, const N: usize>(
        self,
        conn: &mut Connection<'_, T, N>,
    ) -> Result<(), Error<T::Error>>
    where
        T: Read + Write,
    {
        self.write_from_ref(conn).await
    }

    #[inline]
    pub(crate) async fn write_from_ref<T, const N: usize>(
        &self,
        conn: &mut Connection<'_, T, N>,
    ) -> Result<(), Error<T::Error>>
    where
        T: Read + Write,
    {
        conn.initiate_response(
            self.headers.status,
            Some(self.headers.message),
            self.headers.content_type,
        )
        .await?;

        conn.write_all(&self.body.0).await
    }

    const fn new(headers: Headers, body: Body) -> Response {
        Self { headers, body }
    }
}

// A private supertrait used to seal a trait, preventing implementations
// in downstream crates.
mod private {
    #[doc(hidden)]
    pub trait Sealed {}
}

/// Trait for converting a type into an `HTTP` response.
///
/// Types implementing [`IntoResponse`] can be returned directly from
/// request handlers.
///
/// Note: This trait is currently sealed, preventing downstream crates from
/// implementing [`IntoResponse`] for their own types.
pub trait IntoResponse: Sized + private::Sealed {
    /// Converts a type into a [`Response`].
    fn into_response(self) -> Response;
}

#[inline]
fn json_to_vec<T: Serialize>(value: T) -> Vec<u8> {
    match serde_json::to_vec(&value) {
        Ok(value) => value,
        // TODO: A fallback response should be textual and intercepted by
        // the controller. Add a fallback response to Ascot.
        Err(e) => format!("{e:?}").as_bytes().into(),
    }
}

/// An `Ok` response.
///
/// It signals to the controller that the device operation associated with
/// the invoked route has successfully completed.
pub struct OkResponse(Headers);

impl private::Sealed for OkResponse {}

impl IntoResponse for OkResponse {
    fn into_response(self) -> Response {
        let body = Body::owned(json_to_vec(AscotOkResponse::ok()));
        Response::new(self.0, body)
    }
}

impl OkResponse {
    /// Creates an [`OkResponse`].
    #[must_use]
    pub const fn ok() -> Self {
        Self(Headers::ok())
    }
}

// TODO: AscotErrorResponse allocates. We should avoid that providing a plain
// response for embedded systems.
/// A response indicating an error, containing structured details about the
/// issue encountered during the execution of a device operation.
///
/// It includes the type of error, the underlying cause, and additional relevant
/// information about the error.
pub struct ErrorResponse(Headers, AscotErrorResponse);

impl private::Sealed for ErrorResponse {}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Body::owned(json_to_vec(self.1));
        Response::new(self.0, body)
    }
}

impl ErrorResponse {
    /// Generates an [`ErrorResponse`] containing a specific [`ActionError`]
    /// along with a descriptive error message.
    #[must_use]
    #[inline]
    pub fn with_description(error: ActionError, description: &str) -> Self {
        Self(
            Headers::json_error(),
            AscotErrorResponse::with_description(error, description),
        )
    }

    /// Generates an [`ErrorResponse`] containing a specific [`ActionError`],
    /// a descriptive error message, and additional relevant information
    /// about the error.
    #[must_use]
    #[inline]
    pub fn with_description_error(error: ActionError, description: &str, info: &str) -> Self {
        Self(
            Headers::json_error(),
            AscotErrorResponse::with_description_error(error, description, info),
        )
    }

    /// Generates an [`ErrorResponse`] for invalid data along with a
    /// descriptive error message.
    #[must_use]
    #[inline]
    pub fn invalid_data(description: &str) -> Self {
        Self::with_description(ActionError::InvalidData, description)
    }

    /// Generates an [`ErrorResponse`] for an internal error along with
    /// a descriptive error message.
    #[must_use]
    #[inline]
    pub fn internal(description: &str) -> Self {
        Self::with_description(ActionError::Internal, description)
    }

    /// Generates an [`ErrorResponse`] for an internal error with a
    /// descriptive error message, and additional relevant information
    /// about the error.
    #[must_use]
    #[inline]
    pub fn internal_with_error(description: &str, info: &str) -> Self {
        Self::with_description_error(ActionError::Internal, description, info)
    }
}

/// A response that can be serialized for transmission or storage.
///
/// It offers detailed information about the data generated during
/// a device operation.
pub struct SerialResponse<T: Serialize + DeserializeOwned>(Headers, AscotSerialResponse<T>);

impl<T: Serialize + DeserializeOwned> SerialResponse<T> {
    /// Generates a [`SerialResponse`].
    #[inline]
    pub fn new(value: T) -> Self {
        Self(Headers::json(), AscotSerialResponse::new(value))
    }
}

impl<T: Serialize + DeserializeOwned> private::Sealed for SerialResponse<T> {}

impl<T: Serialize + DeserializeOwned> IntoResponse for SerialResponse<T> {
    fn into_response(self) -> Response {
        let body = Body::owned(json_to_vec(self.1));
        Response::new(self.0, body)
    }
}

// TODO: Add this kind of response to the ascot crate. It is necessary that
// a controller receives a response when a method is not correct or a route is
// not found.
pub(crate) struct RouteErrorResponse(Headers);

impl private::Sealed for RouteErrorResponse {}

impl IntoResponse for RouteErrorResponse {
    fn into_response(self) -> Response {
        let body = match self.0.status {
            405 => Body::static_ref("Method not allowed".as_bytes()),
            _ => Body::empty(),
        };
        Response::new(self.0, body)
    }
}

impl RouteErrorResponse {
    #[must_use]
    pub(crate) const fn not_found() -> Self {
        Self(Headers::not_found())
    }

    #[must_use]
    pub(crate) const fn not_allowed() -> Self {
        Self(Headers::not_allowed())
    }
}

pub(crate) struct JsonResponse<'a, T: Serialize>(Headers, &'a T);

impl<'a, T: Serialize> JsonResponse<'a, T> {
    #[inline]
    pub fn new(value: &'a T) -> Self {
        Self(Headers::json(), value)
    }
}

impl<T: Serialize> private::Sealed for JsonResponse<'_, T> {}

impl<T: Serialize> IntoResponse for JsonResponse<'_, T> {
    fn into_response(self) -> Response {
        let body = Body::owned(json_to_vec(self.1));
        Response::new(self.0, body)
    }
}
