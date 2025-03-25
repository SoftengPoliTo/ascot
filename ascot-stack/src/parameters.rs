use heapless::FnvIndexMap;

use serde::{Deserialize, Serialize};

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

/// Route input parameters.
#[derive(Debug, Clone)]
pub struct Parameters<const N: usize>(FnvIndexMap<&'static str, ParameterKind, N>);

impl Parameters<2> {
    /// Creates [`Parameters`] with one [`ParameterKind`].
    #[inline]
    #[must_use]
    pub fn one() -> Self {
        Self::new()
    }

    /// Creates [`Parameters`] with two [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn two() -> Self {
        Self::new()
    }
}

impl Parameters<4> {
    /// Creates [`Parameters`] with three [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn three() -> Self {
        Self::new()
    }

    /// Creates [`Parameters`] with four [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn four() -> Self {
        Self::new()
    }
}

impl Parameters<8> {
    /// Creates [`Parameters`] with five [`ParameterKind`].
    #[inline]
    #[must_use]
    pub fn five() -> Self {
        Self::new()
    }

    /// Creates [`Parameters`] with six [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn six() -> Self {
        Self::new()
    }

    /// Creates [`Parameters`] with seven [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn seven() -> Self {
        Self::new()
    }

    /// Creates [`Parameters`] with eight [`ParameterKind`]s.
    #[inline]
    #[must_use]
    pub fn eight() -> Self {
        Self::new()
    }
}

impl<const N: usize> Parameters<N> {
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
        ParametersData::from(self)
    }

    pub(crate) const fn new() -> Self {
        Self(FnvIndexMap::new())
    }

    fn create_parameter(mut self, name: &'static str, parameter_kind: ParameterKind) -> Self {
        let _ = self.0.insert(name, parameter_kind);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::Parameters;

    #[test]
    fn test_numeric_parameters() {
        let parameters = Parameters::eight()
            .bool("bool", true)
            .u8("u8", 0)
            .u16("u16", 0)
            .u32("u32", 0)
            .u64("u64", 0)
            .f32("f32", 0.)
            .f64("f64", 0.)
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
        let parameters = Parameters::two()
            .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
            .rangef64_with_default("rangef64", (0., 20., 0.1), 5.);

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
