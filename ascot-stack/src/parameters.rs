use heapless::FnvIndexMap;

use serde::Serialize;

use crate::collections::create_map;

/// All supported kinds of route input parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ParameterKind {
    /// A [`bool`] value.
    Bool {
        /// The initial [`bool`] value, but also the default one
        /// in case of missing input parameter.
        default: bool,
    },
    /// An [`u8`] value.
    U8 {
        /// The initial [`u8`] value, but also the default one
        /// in case of a missing input parameter.
        default: u8,
    },
    /// An [`u16`] value.
    U16 {
        /// The initial [`u16`] value, but also the default one
        /// in case of a missing input parameter.
        default: u16,
    },
    /// An [`u32`] value.
    U32 {
        /// The initial [`u32`] value, but also the default one
        /// in case of a missing input parameter.
        default: u32,
    },
    /// An [`u64`] value.
    U64 {
        /// The initial [`u64`] value, but also the default one
        /// in case of a missing input parameter.
        default: u64,
    },
    /// A [`f32`] value.
    F32 {
        /// The initial [`f32`] value, but also the default one
        /// in case of a missing input parameter.
        default: f32,
    },
    /// A [`f64`] value.
    F64 {
        /// The initial [`f64`] value, but also the default one
        /// in case of a missing input.
        default: f64,
    },
    /// A range of [`u64`] values.
    RangeU64 {
        /// Minimum allowed [`u64`] value.
        min: u64,
        /// Maximum allowed [`u64`] value.
        max: u64,
        /// The [`u64`] step to pass from one allowed value to another one
        /// within the range.
        step: u64,
        /// Initial [`u64`] range value.
        default: u64,
    },
    /// A range of [`f64`] values.
    RangeF64 {
        /// Minimum allowed [`f64`] value.
        min: f64,
        /// Maximum allowed [`u64`] value.
        max: f64,
        /// The [`f64`] step to pass from one allowed value to another one
        /// within the range.
        step: f64,
        /// Initial [`f64`] range value.
        default: f64,
    },
}

impl Eq for ParameterKind {}

impl ParameterKind {
    /// Creates a [`bool`] parameter.
    #[must_use]
    #[inline]
    pub fn bool(default: bool) -> Self {
        Self::Bool { default }
    }

    /// Creates an [`u8`] parameter.
    #[must_use]
    #[inline]
    pub fn u8(default: u8) -> Self {
        Self::U8 { default }
    }

    /// Creates an [`u16`] parameter.
    #[must_use]
    #[inline]
    pub fn u16(default: u16) -> Self {
        Self::U16 { default }
    }

    /// Creates an [`u32`] parameter.
    #[must_use]
    #[inline]
    pub fn u32(default: u32) -> Self {
        Self::U32 { default }
    }

    /// Creates an [`u64`] parameter.
    #[must_use]
    #[inline]
    pub fn u64(default: u64) -> Self {
        Self::U64 { default }
    }

    /// Creates a [`f32`] parameter.
    #[must_use]
    #[inline]
    pub fn f32(default: f32) -> Self {
        Self::F32 { default }
    }

    /// Creates a [`f64`] parameter.
    #[must_use]
    #[inline]
    pub fn f64(default: f64) -> Self {
        Self::F64 { default }
    }

    /// Creates an [`u64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64(range: (u64, u64, u64)) -> Self {
        Self::rangeu64_with_default(range, 0)
    }

    /// Creates an [`u64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64_with_default(range: (u64, u64, u64), default: u64) -> Self {
        Self::RangeU64 {
            min: range.0,
            max: range.1,
            step: range.2,
            default,
        }
    }

    /// Creates a [`f64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangef64(range: (f64, f64, f64)) -> Self {
        Self::rangef64_with_default(range, 0.0)
    }

    /// Creates a [`f64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangef64_with_default(range: (f64, f64, f64), default: f64) -> Self {
        Self::RangeF64 {
            min: range.0,
            max: range.1,
            step: range.2,
            default,
        }
    }
}

/// A map of serializable [`Parameters`] data.
#[derive(Debug, Clone, Serialize)]
pub struct ParametersData<const N: usize>(FnvIndexMap<&'static str, ParameterKind, N>);

impl<const N: usize> ParametersData<N> {
    /// Checks whether [`ParametersData`] is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const N: usize> From<Parameters<N>> for ParametersData<N> {
    fn from(parameters: Parameters<N>) -> Self {
        Self(parameters.0)
    }
}

create_map!(
    Parameters,
    (&'static str, ParameterKind),
    parameter,
    parameters
);

impl<const N: usize> Parameters<N> {
    /// Serializes [`Parameters`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> ParametersData<N> {
        ParametersData::from(self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::{ParameterKind, Parameters};

    #[test]
    fn test_numeric_parameters() {
        let parameters = Parameters::eight((
            ("bool", ParameterKind::bool(true)),
            ("u8", ParameterKind::u8(0)),
            ("u16", ParameterKind::u16(0)),
            ("u32", ParameterKind::u32(0)),
            ("u64", ParameterKind::u64(0)),
            ("f32", ParameterKind::f32(0.)),
            ("f64", ParameterKind::f64(0.)),
            // Adds a duplicate to see whether that value is maintained or
            // removed.
            ("u16", ParameterKind::u16(0)),
        ));

        assert_eq!(
            serialize(parameters.serialize_data()),
            json!({
                "bool": {
                    "Bool": {
                        "default": true
                    }
                },
                "f32": {
                    "F32": {
                        "default": 0.0
                    }
                },
                "f64": {
                    "F64": {
                        "default": 0.0
                    }
                },
                "u16": {
                    "U16": {
                        "default": 0
                    }
                },
                "u32": {
                    "U32": {
                        "default": 0
                    }
                },
                "u64": {
                    "U64": {
                        "default": 0
                    }
                },
                "u8": {
                    "U8": {
                        "default": 0
                    }
                }
            })
        );
    }

    #[test]
    fn test_range_parameters() {
        let parameters = Parameters::two((
            (
                "rangeu64",
                ParameterKind::rangeu64_with_default((0, 20, 1), 5),
            ),
            (
                "rangef64",
                ParameterKind::rangef64_with_default((0., 20., 0.1), 5.),
            ),
        ));

        assert_eq!(
            serialize(parameters.serialize_data()),
            json!({
                "rangef64": {
                    "RangeF64": {
                        "default": 5.0,
                        "max": 20.0,
                        "min": 0.0,
                        "step": 0.1
                    }
                },
                "rangeu64": {
                    "RangeU64": {
                        "default": 5,
                        "max": 20,
                        "min": 0,
                        "step": 1
                    }
                }
            })
        );
    }
}
