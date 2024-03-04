use alloc::borrow::Cow;

/// All possible error kinds.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// Service error.
    Service,
    /// Not found address.
    NotFoundAddress,
    /// Serialize/Deserialize error.
    Serialization,
    /// Light error.
    Light,
}

impl ErrorKind {
    pub(crate) const fn description(self) -> &'static str {
        match self {
            ErrorKind::Service => "service error",
            ErrorKind::NotFoundAddress => "not found address",
            ErrorKind::Serialization => "serialization",
            ErrorKind::Light => "light error",
        }
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

/// Library error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: Cow<'static, str>,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.error().fmt(f)
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, info: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            info: info.into(),
        }
    }

    pub(crate) fn error(&self) -> String {
        format!("{}: {}", self.kind, self.info)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::new(ErrorKind::Serialization, e.to_string())
    }
}

/// A specialized [`Result`] type for [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
