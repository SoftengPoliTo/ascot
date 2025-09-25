use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use edge_http::io::server::Connection;
use edge_http::io::Error;

use embedded_io_async::{Read, Write};

use serde::Serialize;

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

    const fn text() -> Self {
        Self {
            status: 200,
            message: "Ok",
            content_type: &[("Content-Type", "text/plain")],
        }
    }

    const fn json() -> Self {
        Self {
            status: 200,
            message: "Ok",
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

/// A response with no content.
pub struct EmptyResponse(Headers);

impl private::Sealed for EmptyResponse {}

impl IntoResponse for EmptyResponse {
    fn into_response(self) -> Response {
        let body = match self.0.status {
            200 => Body::static_ref("OK".as_bytes()),
            405 => Body::static_ref("Method not allowed".as_bytes()),
            _ => Body::empty(),
        };
        Response::new(self.0, body)
    }
}

impl EmptyResponse {
    /// Creates an [`EmptyResponse`] with an `Ok` status.
    #[must_use]
    pub const fn ok() -> Self {
        Self(Headers::ok())
    }

    /// Creates an [`EmptyResponse`] with a `NotFound` status.
    #[must_use]
    pub const fn not_found() -> Self {
        Self(Headers::not_found())
    }

    /// Creates an [`EmptyResponse`] with a `NotAllowed` status.
    #[must_use]
    pub const fn not_allowed() -> Self {
        Self(Headers::not_allowed())
    }
}

/// A textual response.
pub struct TextResponse {
    headers: Headers,
    text: &'static str,
}

impl TextResponse {
    /// Creates a [`TextResponse`] with an `Ok` status.
    #[must_use]
    pub const fn new(text: &'static str) -> Self {
        Self {
            headers: Headers::text(),
            text,
        }
    }
}

impl private::Sealed for TextResponse {}

impl IntoResponse for TextResponse {
    fn into_response(self) -> Response {
        Response::new(self.headers, Body::static_ref(self.text.as_bytes()))
    }
}

/// A `json` response.
pub struct JsonResponse<'a, T: Serialize> {
    headers: Headers,
    json: &'a T,
}

impl<'a, T: Serialize> JsonResponse<'a, T> {
    /// Creates a [`JsonResponse`] with an `Ok` status.
    #[inline]
    pub fn new(json: &'a T) -> Self {
        Self {
            headers: Headers::json(),
            json,
        }
    }
}

impl<T: Serialize> private::Sealed for JsonResponse<'_, T> {}

impl<T: Serialize> IntoResponse for JsonResponse<'_, T> {
    fn into_response(self) -> Response {
        let bytes = match serde_json::to_vec(self.json) {
            Ok(value) => value,
            Err(e) => format!("{e:?}").as_bytes().into(),
        };

        Response::new(self.headers, Body::owned(bytes))
    }
}
