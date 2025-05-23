// FIXME: Remove once passing by value will be supported in serde.
#![allow(clippy::trivially_copy_pass_by_ref)]

use alloc::borrow::Cow;
use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::collections::{Map, OutputMap};

fn is_u8_max(value: &u8) -> bool {
    *value == u8::MAX
}

fn is_u8_min(value: &u8) -> bool {
    *value == u8::MIN
}

fn u8_max() -> u8 {
    u8::MAX
}

fn is_u16_max(value: &u16) -> bool {
    *value == u16::MAX
}

fn is_u16_min(value: &u16) -> bool {
    *value == u16::MIN
}

fn u16_max() -> u16 {
    u16::MAX
}

fn is_u32_max(value: &u32) -> bool {
    *value == u32::MAX
}

fn is_u32_min(value: &u32) -> bool {
    *value == u32::MIN
}

fn u32_max() -> u32 {
    u32::MAX
}

fn is_u64_max(value: &u64) -> bool {
    *value == u64::MAX
}

fn is_u64_min(value: &u64) -> bool {
    *value == u64::MIN
}

fn u64_max() -> u64 {
    u64::MAX
}

fn is_f32_max(value: &f32) -> bool {
    *value == f32::MAX
}

fn is_f32_min(value: &f32) -> bool {
    *value == f32::MIN
}

fn f32_min() -> f32 {
    f32::MIN
}

fn f32_max() -> f32 {
    f32::MAX
}

fn is_f64_max(value: &f64) -> bool {
    *value == f64::MAX
}

fn is_f64_min(value: &f64) -> bool {
    *value == f64::MIN
}

fn f64_min() -> f64 {
    f64::MIN
}

fn f64_max() -> f64 {
    f64::MAX
}

/// All supported kinds of route input parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        /// The minimum [`u8`] value allowed.
        #[serde(skip_serializing_if = "is_u8_max")]
        #[serde(default)]
        min: u8,
        /// The maximum [`u8`] value allowed.
        #[serde(skip_serializing_if = "is_u8_min")]
        #[serde(default = "u8_max")]
        max: u8,
    },
    /// An [`u16`] value.
    U16 {
        /// The initial [`u16`] value, but also the default one
        /// in case of a missing input parameter.
        default: u16,
        /// The minimum [`u16`] value allowed.
        #[serde(skip_serializing_if = "is_u16_max")]
        #[serde(default)]
        min: u16,
        /// The maximum [`u16`] value allowed.
        #[serde(skip_serializing_if = "is_u16_min")]
        #[serde(default = "u16_max")]
        max: u16,
    },
    /// An [`u32`] value.
    U32 {
        /// The initial [`u32`] value, but also the default one
        /// in case of a missing input parameter.
        default: u32,
        /// The minimum [`u32`] value allowed.
        #[serde(skip_serializing_if = "is_u32_max")]
        #[serde(default)]
        min: u32,
        /// The maximum [`u32`] allowed value.
        #[serde(skip_serializing_if = "is_u32_min")]
        #[serde(default = "u32_max")]
        max: u32,
    },
    /// An [`u64`] value.
    U64 {
        /// The initial [`u64`] value, but also the default one
        /// in case of a missing input parameter.
        default: u64,
        /// The minimum [`u64`] value allowed.
        #[serde(skip_serializing_if = "is_u64_max")]
        #[serde(default)]
        min: u64,
        /// The maximum [`u64`] allowed value.
        #[serde(skip_serializing_if = "is_u64_min")]
        #[serde(default = "u64_max")]
        max: u64,
    },
    /// A [`f32`] value.
    F32 {
        /// The initial [`f32`] value, but also the default one
        /// in case of a missing input parameter.
        default: f32,
        /// The minimum [`f32`] value allowed.
        #[serde(skip_serializing_if = "is_f32_max")]
        #[serde(default = "f32_min")]
        min: f32,
        /// The maximum [`f32`] allowed value.
        #[serde(skip_serializing_if = "is_f32_min")]
        #[serde(default = "f32_max")]
        max: f32,
        /// The decimal step associated with the [`f32`] value.
        #[serde(skip_serializing_if = "is_f32_min")]
        #[serde(default)]
        step: f32,
    },
    /// A [`f64`] value.
    F64 {
        /// The initial [`f64`] value, but also the default one
        /// in case of a missing input.
        default: f64,
        /// The minimum [`f64`] value allowed.
        #[serde(skip_serializing_if = "is_f64_max")]
        #[serde(default = "f64_min")]
        min: f64,
        /// The maximum [`f64`] allowed value.
        #[serde(skip_serializing_if = "is_f64_min")]
        #[serde(default = "f64_max")]
        max: f64,
        /// The decimal step associated with the [`f64`] value.
        #[serde(skip_serializing_if = "is_f64_min")]
        #[serde(default)]
        step: f64,
    },
    /// A range of [`u64`] values.
    RangeU64 {
        /// Minimum [`u64`] value allowed.
        min: u64,
        /// Maximum [`u64`] value allowed.
        max: u64,
        /// The [`u64`] step necessary to pass from one allowed value
        /// to another one in the range.
        step: u64,
        /// Initial [`u64`] range value.
        default: u64,
    },
    /// A range of [`f64`] values.
    RangeF64 {
        /// The minimum [`f64`] value allowed.
        min: f64,
        /// Maximum [`u64`] value allowed.
        max: f64,
        /// The [`f64`] step necessary to pass from one allowed value
        /// to another one in the range. It is always a positive value.
        step: f64,
        /// Initial [`f64`] range value.
        default: f64,
    },
    /// A characters sequence.
    ///
    /// This kind of input parameter can contain an unknown or a precise
    /// sequence of characters.
    CharsSequence {
        /// Initial characters sequence, which also represents the default
        /// value.
        default: Cow<'static, str>,
        /// The character sequence length.
        length: usize,
    },
}

/// Floating point decimal precision.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DecimalPrecision {
    /// One digit.
    OneDigit,
    /// Two digits.
    TwoDigits,
    /// Three digits.
    ThreeDigits,
    /// Any digits.
    Any,
}

impl DecimalPrecision {
    const fn to_f32(self) -> f32 {
        match self {
            Self::OneDigit => 0.1,
            Self::TwoDigits => 0.01,
            Self::ThreeDigits => 0.001,
            Self::Any => 0.,
        }
    }

    const fn to_f64(self) -> f64 {
        match self {
            Self::OneDigit => 0.1,
            Self::TwoDigits => 0.01,
            Self::ThreeDigits => 0.001,
            Self::Any => 0.,
        }
    }
}

/// A map of serializable and deserializable [`Parameters`] data.
pub type ParametersData = OutputMap<String, ParameterKind>;

/// Route input parameters.
#[derive(Debug, Clone)]
pub struct Parameters(Map<&'static str, ParameterKind>);

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Parameters {
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
        self.create_parameter(
            name,
            ParameterKind::U8 {
                default,
                min: u8::MAX,
                max: u8::MIN,
            },
        )
    }

    /// Adds an [`u8`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn u8_with_limits(self, name: &'static str, default: u8, min: u8, max: u8) -> Self {
        self.create_parameter(name, ParameterKind::U8 { default, min, max })
    }

    /// Adds an [`u16`] parameter.
    #[must_use]
    #[inline]
    pub fn u16(self, name: &'static str, default: u16) -> Self {
        self.create_parameter(
            name,
            ParameterKind::U16 {
                default,
                min: u16::MAX,
                max: u16::MIN,
            },
        )
    }

    /// Adds an [`u16`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn u16_with_limits(self, name: &'static str, default: u16, min: u16, max: u16) -> Self {
        self.create_parameter(name, ParameterKind::U16 { default, min, max })
    }

    /// Adds an [`u32`] parameter.
    #[must_use]
    #[inline]
    pub fn u32(self, name: &'static str, default: u32) -> Self {
        self.create_parameter(
            name,
            ParameterKind::U32 {
                default,
                min: u32::MAX,
                max: u32::MIN,
            },
        )
    }

    /// Adds an [`u32`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn u32_with_limits(self, name: &'static str, default: u32, min: u32, max: u32) -> Self {
        self.create_parameter(name, ParameterKind::U32 { default, min, max })
    }

    /// Adds an [`u64`] parameter.
    #[must_use]
    #[inline]
    pub fn u64(self, name: &'static str, default: u64) -> Self {
        self.create_parameter(
            name,
            ParameterKind::U64 {
                default,
                min: u64::MAX,
                max: u64::MIN,
            },
        )
    }

    /// Adds an [`u64`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn u64_with_limits(self, name: &'static str, default: u64, min: u64, max: u64) -> Self {
        self.create_parameter(name, ParameterKind::U64 { default, min, max })
    }

    /// Adds a [`f32`] parameter.
    #[must_use]
    #[inline]
    pub fn f32(self, name: &'static str, default: f32) -> Self {
        self.create_parameter(
            name,
            ParameterKind::F32 {
                default,
                min: f32::MAX,
                max: f32::MIN,
                step: 0.,
            },
        )
    }

    /// Adds a [`f32`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn f32_with_limits(
        self,
        name: &'static str,
        default: f32,
        min: f32,
        max: f32,
        decimal_precision: DecimalPrecision,
    ) -> Self {
        self.create_parameter(
            name,
            ParameterKind::F32 {
                default,
                min,
                max,
                step: decimal_precision.to_f32(),
            },
        )
    }

    /// Adds a [`f64`] parameter.
    #[must_use]
    #[inline]
    pub fn f64(self, name: &'static str, default: f64) -> Self {
        self.create_parameter(
            name,
            ParameterKind::F64 {
                default,
                min: f64::MAX,
                max: f64::MIN,
                step: 0.,
            },
        )
    }

    /// Adds a [`f64`] parameter with limits.
    #[must_use]
    #[inline]
    pub fn f64_with_limits(
        self,
        name: &'static str,
        default: f64,
        min: f64,
        max: f64,
        decimal_precision: DecimalPrecision,
    ) -> Self {
        self.create_parameter(
            name,
            ParameterKind::F64 {
                default,
                min,
                max,
                step: decimal_precision.to_f64(),
            },
        )
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
                step: range.2.abs(),
                default,
            },
        )
    }

    /// Adds a characters sequence with a determined length.
    #[must_use]
    #[inline]
    pub fn characters_sequence(
        self,
        name: &'static str,
        default: impl Into<Cow<'static, str>>,
    ) -> Self {
        let default = default.into();
        self.create_parameter(
            name,
            ParameterKind::CharsSequence {
                length: default.len(),
                default,
            },
        )
    }

    /// Serializes [`Parameters`] data.
    ///
    /// It consumes the data.
    #[must_use]
    #[inline]
    pub fn serialize_data(self) -> ParametersData {
        let mut data = ParametersData::new();
        for (key, value) in self.0 {
            data.add(key.into(), value);
        }
        data
    }

    fn create_parameter(self, name: &'static str, parameter_kind: ParameterKind) -> Self {
        Self(self.0.insert(name, parameter_kind))
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::{deserialize, serialize};

    use super::{OutputMap, ParameterKind, Parameters, ParametersData};

    fn expected_parameters_data() -> OutputMap<String, ParameterKind> {
        OutputMap::new()
            .insert("bool".into(), ParameterKind::Bool { default: true })
            .insert(
                "u8".into(),
                ParameterKind::U8 {
                    default: 0,
                    min: u8::MIN,
                    max: u8::MAX,
                },
            )
            .insert(
                "u16".into(),
                ParameterKind::U16 {
                    default: 0,
                    min: u16::MIN,
                    max: u16::MAX,
                },
            )
            .insert(
                "u32".into(),
                ParameterKind::U32 {
                    default: 0,
                    min: u32::MIN,
                    max: u32::MAX,
                },
            )
            .insert(
                "u64".into(),
                ParameterKind::U64 {
                    default: 0,
                    min: u64::MIN,
                    max: u64::MAX,
                },
            )
            .insert(
                "f32".into(),
                ParameterKind::F32 {
                    default: 0.,
                    min: f32::MIN,
                    max: f32::MAX,
                    step: 0.,
                },
            )
            .insert(
                "f64".into(),
                ParameterKind::F64 {
                    default: 0.,
                    min: f64::MIN,
                    max: f64::MAX,
                    step: 0.,
                },
            )
            .insert(
                "rangeu64".into(),
                ParameterKind::RangeU64 {
                    min: 0,
                    max: 20,
                    step: 1,
                    default: 5,
                },
            )
            .insert(
                "rangef64".into(),
                ParameterKind::RangeF64 {
                    min: 0.,
                    max: 20.,
                    step: 0.1,
                    default: 5.,
                },
            )
            .insert(
                "greeting".into(),
                ParameterKind::CharsSequence {
                    default: "hello".into(),
                    length: 5,
                },
            )
            .insert(
                "greeting2".into(),
                ParameterKind::CharsSequence {
                    default: "hello".into(),
                    length: 5,
                },
            )
    }

    #[test]
    fn test_parameters() {
        let parameters = Parameters::new()
            .bool("bool", true)
            .u8("u8", 0)
            .u16("u16", 0)
            .u32("u32", 0)
            .u64("u64", 0)
            .f32("f32", 0.)
            .f64("f64", 0.)
            .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
            .rangef64_with_default("rangef64", (0., 20., 0.1), 5.)
            .characters_sequence("greeting", "hello")
            .characters_sequence("greeting2", String::from("hello"))
            // Adds a duplicate to see whether that value is maintained or
            // removed.
            .u16("u16", 0);

        assert_eq!(
            deserialize::<ParametersData>(serialize(parameters.serialize_data())),
            expected_parameters_data(),
        );
    }
}
