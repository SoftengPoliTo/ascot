use ascot::response::{InfoResponse, OkResponse, SerialResponse};

use reqwest::Response as ReqwestResponse;

use serde::{Serialize, de::DeserializeOwned};

use bytes::Bytes;
use futures_util::{Stream, TryStreamExt};

use crate::error::{Error, ErrorKind, Result};

// TODO:
// OkCollector --> Save Ok responses in order to maintain a history.
// SerialCollector --> Save serial responses in order to maintain a history.
// InfoCollector --> Save Info responses in order to maintain a history.
// StreamCollector --> Save information about a Stream Response before and after

async fn json_response<T>(response: ReqwestResponse) -> Result<T>
where
    T: Serialize + DeserializeOwned,
{
    response
        .json::<T>()
        .await
        .map_err(|e| Error::new(ErrorKind::JsonResponse, format!("Json error caused by {e}")))
}

/// An [`OkResponse`] body parser.
pub struct OkResponseParser(ReqwestResponse);

impl OkResponseParser {
    /// Parses the internal response body with the intent of retrieving
    /// an [`OkResponse`].
    ///
    /// # Errors
    ///
    /// The response body does not contain a valid [`OkResponse`].
    /// A parsing error is raised either because the given format is not correct
    /// or because binary data contains some syntactic or semantic errors.
    pub async fn parse_body(self) -> Result<OkResponse> {
        json_response::<OkResponse>(self.0).await
    }

    pub(crate) const fn new(response: ReqwestResponse) -> Self {
        Self(response)
    }
}

/// A [`SerialResponse`] body parser.
pub struct SerialResponseParser(ReqwestResponse);

impl SerialResponseParser {
    /// Parses the internal response body with the intent of retrieving
    /// a [`SerialResponse`].
    ///
    /// # Errors
    ///
    /// The response body does not contain a valid [`SerialResponse`].
    /// A parsing error is raised either because the given format is not correct
    /// or because binary data contains some syntactic or semantic errors.
    pub async fn parse_body<T: Serialize + DeserializeOwned>(self) -> Result<SerialResponse<T>> {
        json_response::<SerialResponse<T>>(self.0).await
    }

    pub(crate) const fn new(response: ReqwestResponse) -> Self {
        Self(response)
    }
}

/// An [`InfoResponse`] body parser.
pub struct InfoResponseParser(ReqwestResponse);

impl InfoResponseParser {
    /// Parses the internal response body with the intent of retrieving
    /// an [`InfoResponse`].
    ///
    /// # Errors
    ///
    /// The response body does not contain a valid [`InfoResponse`].
    /// A parsing error is raised either because the given format is not correct
    /// or because binary data contains some syntactic or semantic errors.
    pub async fn parse_body(self) -> Result<InfoResponse> {
        json_response::<InfoResponse>(self.0).await
    }

    pub(crate) const fn new(response: ReqwestResponse) -> Self {
        Self(response)
    }
}

/// A stream response.
pub struct StreamResponse(ReqwestResponse);

impl StreamResponse {
    /// Consumes the internal response body opening a bytes stream.
    ///
    /// # Errors
    ///
    /// Stream data are not retrieved correctly because of network failures or
    /// data corruption.
    pub fn open_stream(self) -> impl Stream<Item = Result<Bytes>> {
        self.0.bytes_stream().map_err(|e| {
            Error::new(
                ErrorKind::StreamResponse,
                format!("Stream error caused by {e}"),
            )
        })
    }

    pub(crate) const fn new(response: ReqwestResponse) -> Self {
        Self(response)
    }
}

/// All supported device responses kinds.
///
/// Every response  a specific body parser to analyze
/// its internal data.
pub enum Response {
    /// A skipped response occurs when a request has not been sent because of
    /// privacy policy rules.
    Skipped,
    /// An [`OkResponse`] body.
    OkBody(OkResponseParser),
    /// A [`SerialResponse`] body.
    SerialBody(SerialResponseParser),
    /// An [`InfoResponse`] body.
    InfoBody(InfoResponseParser),
    /// A stream response body.
    StreamBody(StreamResponse),
}
