/// All possible error kinds.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// Error creating a fixed-size text.
    FixedText,
}

impl ErrorKind {
    pub(crate) const fn description(self) -> &'static str {
        match self {
            Self::FixedText => "Fixed-size text",
        }
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

/// General error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: &'static str,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, info: &'static str) -> Self {
        Self { kind, info }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}: {} ", self.kind, self.info)
    }
}

/// A specialized [`Result`] type.
pub type Result<T> = core::result::Result<T, Error>;
