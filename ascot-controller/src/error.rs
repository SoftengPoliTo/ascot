use std::borrow::Cow;

/// All possible error kinds.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    /// Errors caused by the discovery process.
    Discovery,
    /// Errors caused by sending requests to a device.
    Request,
    /// Errors caused by a wrong input parameter.
    WrongParameter,
    /// Errors in receiving a json response.
    JsonResponse,
    /// Errors in receiving a bytes stream response.
    StreamResponse,
    /// Errors in building the mechanism to send a request to a device.
    Sender,
}

impl ErrorKind {
    pub(crate) const fn description(self) -> &'static str {
        match self {
            Self::Discovery => "Discovery",
            Self::Request => "Request",
            Self::WrongParameter => "Wrong Parameter",
            Self::JsonResponse => "Json Response",
            Self::StreamResponse => "Stream Response",
            Self::Sender => "Response Sender",
        }
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

/// Controller error.
#[derive(PartialEq)]
pub struct Error {
    kind: ErrorKind,
    description: Cow<'static, str>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Error {
    /// Creates an [`Error`] from an [`ErrorKind`] and a description.
    #[inline]
    pub fn new(kind: ErrorKind, description: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            description: description.into(),
        }
    }

    fn format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self.kind)?;
        write!(f, "Cause: {}", self.description)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::new(ErrorKind::Request, e.to_string())
    }
}

impl From<mdns_sd::Error> for Error {
    fn from(e: mdns_sd::Error) -> Self {
        Self::new(ErrorKind::Discovery, e.to_string())
    }
}

/// A specialized [`Result`] type for [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
