use alloc::borrow::Cow;
use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::collections::{Map, OutputMap};

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
    /// A characters sequence.
    ///
    /// This kind of input parameter can contain an unknown or a precise
    /// sequence of characters.
    CharsSequence {
        /// Initial characters sequence, which also represents the default
        /// value.
        default: Cow<'static, str>,
    },
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

    /// Adds a characters sequence.
    #[must_use]
    #[inline]
    pub fn characters_sequence(
        self,
        name: &'static str,
        default: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.create_parameter(
            name,
            ParameterKind::CharsSequence {
                default: default.into(),
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
    use crate::alloc::string::ToString;

    use crate::{deserialize, serialize};

    use super::{OutputMap, ParameterKind, Parameters, ParametersData};

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
            .characters_sequence("greeting2", "hello".to_string())
            // Adds a duplicate to see whether that value is maintained or
            // removed.
            .u16("u16", 0);

        let parameters_data = OutputMap::new()
            .insert("bool".into(), ParameterKind::Bool { default: true })
            .insert("u8".into(), ParameterKind::U8 { default: 0 })
            .insert("u16".into(), ParameterKind::U16 { default: 0 })
            .insert("u32".into(), ParameterKind::U32 { default: 0 })
            .insert("u64".into(), ParameterKind::U64 { default: 0 })
            .insert("f32".into(), ParameterKind::F32 { default: 0. })
            .insert("f64".into(), ParameterKind::F64 { default: 0. })
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
                },
            )
            .insert(
                "greeting2".into(),
                ParameterKind::CharsSequence {
                    default: "hello".into(),
                },
            );

        assert_eq!(
            deserialize::<ParametersData>(serialize(parameters.serialize_data())),
            parameters_data,
        );
    }
}
