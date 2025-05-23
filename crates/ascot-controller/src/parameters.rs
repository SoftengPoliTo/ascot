use ascot::collections::Map;
use ascot::parameters::{ParameterKind, ParametersData};

use tracing::error;

use crate::error::{Error, ErrorKind};

pub(crate) fn parameter_error(message: String) -> Error {
    error!(message);
    Error::new(ErrorKind::WrongParameter, message)
}

const fn str_type(parameter_kind: &ParameterKind) -> &'static str {
    match parameter_kind {
        ParameterKind::Bool { .. } => "bool",
        ParameterKind::U8 { .. } => "u8",
        ParameterKind::U16 { .. } => "u16",
        ParameterKind::U32 { .. } => "u32",
        ParameterKind::U64 { .. } | ParameterKind::RangeU64 { .. } => "u64",
        ParameterKind::F32 { .. } => "f32",
        ParameterKind::F64 { .. } | ParameterKind::RangeF64 { .. } => "f64",
        ParameterKind::CharsSequence { .. } => "String",
    }
}

pub(crate) fn convert_to_parameter_value(parameter_kind: &ParameterKind) -> Option<ParameterValue> {
    match parameter_kind {
        ParameterKind::Bool { default } => Some(ParameterValue::Bool(*default)),
        ParameterKind::U8 { default, .. } => Some(ParameterValue::U8(*default)),
        ParameterKind::U16 { default, .. } => Some(ParameterValue::U16(*default)),
        ParameterKind::U32 { default, .. } => Some(ParameterValue::U32(*default)),
        ParameterKind::U64 { default, .. } | ParameterKind::RangeU64 { default, .. } => {
            Some(ParameterValue::U64(*default))
        }
        ParameterKind::F32 { default, .. } => Some(ParameterValue::F32(*default)),
        ParameterKind::F64 { default, .. } | ParameterKind::RangeF64 { default, .. } => {
            Some(ParameterValue::F64(*default))
        }
        ParameterKind::CharsSequence { default, .. } => {
            Some(ParameterValue::String(default.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ParameterValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
}

impl ParameterValue {
    pub(crate) fn as_string(&self) -> String {
        match self {
            Self::Bool(v) => v.to_string(),
            Self::U8(v) => v.to_string(),
            Self::U16(v) => v.to_string(),
            Self::U32(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
            Self::F32(v) => v.to_string(),
            Self::F64(v) => v.to_string(),
            Self::String(v) => v.to_string(),
        }
    }

    const fn compare_with_kind(&self, parameter_kind: &ParameterKind) -> bool {
        matches!(
            (self, parameter_kind),
            (Self::Bool(_), ParameterKind::Bool { .. })
                | (Self::U8(_), ParameterKind::U8 { .. })
                | (Self::U16(_), ParameterKind::U16 { .. })
                | (Self::U32(_), ParameterKind::U32 { .. })
                | (
                    Self::U64(_),
                    ParameterKind::U64 { .. } | ParameterKind::RangeU64 { .. }
                )
                | (Self::F32(_), ParameterKind::F32 { .. })
                | (
                    Self::F64(_),
                    ParameterKind::F64 { .. } | ParameterKind::RangeF64 { .. }
                )
                | (Self::String(_), ParameterKind::CharsSequence { .. })
        )
    }
}

/// Route input parameters.
#[derive(Debug, Clone)]
pub struct Parameters<'a>(Map<&'a str, ParameterValue>);

impl Default for Parameters<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Parameters<'a> {
    /// Creates [`Parameters`].
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self(Map::new())
    }

    /// Adds a [`bool`] value.
    #[inline]
    pub fn bool(&mut self, name: &'a str, value: bool) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::Bool(value))
    }

    /// Adds an [`u8`] parameter.
    #[inline]
    pub fn u8(&mut self, name: &'a str, value: u8) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::U8(value))
    }

    /// Adds an [`u16`] parameter.
    #[inline]
    pub fn u16(&mut self, name: &'a str, value: u16) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::U16(value))
    }

    /// Adds an [`u32`] parameter.
    #[inline]
    pub fn u32(&mut self, name: &'a str, value: u32) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::U32(value))
    }

    /// Adds an [`u64`] parameter.
    #[inline]
    pub fn u64(&mut self, name: &'a str, value: u64) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::U64(value))
    }

    /// Adds a [`f32`] parameter.
    #[inline]
    pub fn f32(&mut self, name: &'a str, value: f32) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::F32(value))
    }

    /// Adds a [`f64`] parameter.
    #[inline]
    pub fn f64(&mut self, name: &'a str, value: f64) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::F64(value))
    }

    /// Adds a characters sequence.
    #[inline]
    pub fn characters_sequence(&mut self, name: &'a str, value: String) -> &mut Self {
        self.add_value_parameter(name, ParameterValue::String(value))
    }

    pub(crate) fn get<'b>(&'b self, name: &'b str) -> Option<&'b ParameterValue> {
        self.0.get(name)
    }

    pub(crate) fn check_parameters(&self, parameters_data: &ParametersData) -> Result<(), Error> {
        for (name, parameter_value) in &self.0 {
            let Some(parameter_kind) = parameters_data.get(*name) else {
                return Err(parameter_error(format!("`{name}` does not exist")));
            };

            if !parameter_value.compare_with_kind(parameter_kind) {
                return Err(parameter_error(format!(
                    "`{name}` must be of type `{}`",
                    str_type(parameter_kind),
                )));
            }
        }
        Ok(())
    }

    fn add_value_parameter(&mut self, name: &'a str, parameter_value: ParameterValue) -> &mut Self {
        self.0.add(name, parameter_value);
        self
    }
}
