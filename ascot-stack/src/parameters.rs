use serde::{Deserialize, Serialize};

use crate::collections::{Map, SerialMap};

/// All supported kinds of route input parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

/// A map of serializable [`Parameters`] data.
pub type ParametersData<const N: usize> = SerialMap<&'static str, ParameterKind, N>;

/// Route input parameters.
#[derive(Debug, Clone)]
pub struct Parameters<const N: usize>(Map<&'static str, ParameterKind, N>);

impl<const N: usize> Default for Parameters<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Parameters<N> {
    /// Creates a [`Parameters`].
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self(Map::new())
    }

    /// Adds a [`bool`] parameter.
    #[must_use]
    #[inline]
    pub fn bool(self, name: &'static str, default: bool) -> Self {
        self.create_parameter(name, ParameterKind::Bool { default })
    }

    /// Adds an [`u8`] parameter.
    #[must_use]
    #[inline]
    pub fn u8(self, name: &'static str, default: u8) -> Self {
        self.create_parameter(name, ParameterKind::U8 { default })
    }

    /// Adds an [`u16`] parameter.
    #[must_use]
    #[inline]
    pub fn u16(self, name: &'static str, default: u16) -> Self {
        self.create_parameter(name, ParameterKind::U16 { default })
    }

    /// Adds an [`u32`] parameter.
    #[must_use]
    #[inline]
    pub fn u32(self, name: &'static str, default: u32) -> Self {
        self.create_parameter(name, ParameterKind::U32 { default })
    }

    /// Adds an [`u64`] parameter.
    #[must_use]
    #[inline]
    pub fn u64(self, name: &'static str, default: u64) -> Self {
        self.create_parameter(name, ParameterKind::U64 { default })
    }

    /// Adds a [`f32`] parameter.
    #[must_use]
    #[inline]
    pub fn f32(self, name: &'static str, default: f32) -> Self {
        self.create_parameter(name, ParameterKind::F32 { default })
    }

    /// Adds a [`f64`] parameter.
    #[must_use]
    #[inline]
    pub fn f64(self, name: &'static str, default: f64) -> Self {
        self.create_parameter(name, ParameterKind::F64 { default })
    }

    /// Adds an [`u64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64(self, name: &'static str, range: (u64, u64, u64)) -> Self {
        self.rangeu64_with_default(name, range, 0)
    }

    /// Adds an [`u64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64_with_default(
        self,
        name: &'static str,
        range: (u64, u64, u64),
        default: u64,
    ) -> Self {
        self.create_parameter(
            name,
            ParameterKind::RangeU64 {
                min: range.0,
                max: range.1,
                step: range.2,
                default,
            },
        )
    }

    /// Adds a [`f64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangef64(self, name: &'static str, range: (f64, f64, f64)) -> Self {
        self.rangef64_with_default(name, range, 0.0)
    }

    /// Adds a [`f64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangef64_with_default(
        self,
        name: &'static str,
        range: (f64, f64, f64),
        default: f64,
    ) -> Self {
        self.create_parameter(
            name,
            ParameterKind::RangeF64 {
                min: range.0,
                max: range.1,
                step: range.2,
                default,
            },
        )
    }

    /// Serializes [`Parameters`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> ParametersData<N> {
        let mut data = ParametersData::new();
        for (key, value) in &self.0 {
            data.add(key, *value);
        }
        data
    }

    fn create_parameter(self, name: &'static str, parameter_kind: ParameterKind) -> Self {
        Self(self.0.insert(name, parameter_kind))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::Parameters;

    #[test]
    fn test_parameters() {
        let parameters = Parameters::<16>::new()
            .bool("bool", true)
            .u8("u8", 0)
            .u16("u16", 0)
            .u32("u32", 0)
            .u64("u64", 0)
            .f32("f32", 0.)
            .f64("f64", 0.)
            .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
            .rangef64_with_default("rangef64", (0., 20., 0.1), 5.)
            // Adds a duplicate to see whether that value is maintained or
            // removed.
            .u16("u16", 0);

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
}
