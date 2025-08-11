use alloc::borrow::Cow;
use alloc::string::String;
use alloc::format;

/// All possible error kinds.
#[derive(Copy, Clone)]
pub enum ErrorKind {
    /// Wi-Fi connection error.
    WiFi,
    /// mDNS error.
    Mdns,
    /// Esp32-C3 internal error.
    Esp32C3,
    /// Esp32-C3 input/output error.
    Esp32C3IO,
    /// Light error.
    Light,
}

impl ErrorKind {
    const fn description(self) -> &'static str {
        match self {
            ErrorKind::WiFi => "Wi-Fi",
            ErrorKind::Mdns => "Mdns",
            ErrorKind::Esp32C3 => "Esp32-C3 internal error",
            ErrorKind::Esp32C3IO => "Esp32-C3 input/output",
            ErrorKind::Light => "Light device",
        }
    }
}

impl core::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

/// A firmware error.
pub struct Error {
    kind: ErrorKind,
    info: Cow<'static, str>,
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.error().fmt(f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.error().fmt(f)
    }
}

impl Error {
    #[inline]
    pub(crate) fn new(kind: ErrorKind, info: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            info: info.into(),
        }
    }

    #[inline]
    fn error(&self) -> String {
        format!("{}: {}", self.kind, self.info)
    }
}

impl From<esp_wifi::wifi::WifiError> for Error {
    fn from(e: esp_wifi::wifi::WifiError) -> Self {
        Self::new(ErrorKind::WiFi, format!("{e:?}"))
    }
}

impl<E: core::fmt::Debug> From<edge_mdns::io::MdnsIoError<E>> for Error {
    fn from(e: edge_mdns::io::MdnsIoError<E>) -> Self {
        Self::new(ErrorKind::Mdns, format!("{e:?}"))
    }
}

/// A specialized [`Result`] type for [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
