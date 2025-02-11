use ascot_library::response::{InfoResponse, OkResponse, SerialResponse};

use reqwest::Response as ReqwestResponse;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use bytes::Bytes;
use futures_util::{Stream, TryStreamExt};

use crate::error::{Error, ErrorKind, Result};

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
    pub async fn parse_body(self) -> Result<SerialResponse<Value>> {
        json_response::<SerialResponse<Value>>(self.0).await
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

/// All supported kinds of device responses.
///
/// Every response presents a specific body parser to analyze its internal data.
pub enum Response {
    /// An [`OkResponse`].
    Ok(OkResponseParser),
    /// A [`SerialResponse`].
    Serial(SerialResponseParser),
    /// An [`InfoResponse`].
    Info(InfoResponseParser),
    /// A stream response.
    Stream(StreamResponse),
}

// OkCollector --> Save Ok responses in order to maintain a history.
// SerialCollector --> Save serial responses in order to maintain a history.
// InfoCollector --> Save Info responses in order to maintain a history.
// StreamCollector --> Save information about a Stream Response before and after
// running the bytes stream.
