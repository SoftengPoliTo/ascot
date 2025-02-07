use serde::{Deserialize, Serialize};

use crate::utils::collections::{Collection, SerialCollection};

/// An [`Input`] structure.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InputStructure {
    /// A [`bool`] value.
    Bool {
        /// Both the initial [`bool`] value, but also the default one
        /// in case of missing input.
        default: bool,
    },
    /// A [`u8`] value.
    U8 {
        /// Both the initial [`u8`] value, but also the default one
        /// in case of missing input.
        default: u8,
    },
    /// A [`u16`] value.
    U16 {
        /// Both the initial [`u16`] value, but also the default one
        /// in case of missing input.
        default: u16,
    },
    /// A [`u32`] value.
    U32 {
        /// Both the initial [`u32`] value, but also the default one
        /// in case of missing input.
        default: u32,
    },
    /// A [`u64`] value.
    U64 {
        /// Both the initial [`u64`] value, but also the default one
        /// in case of missing input.
        default: u64,
    },
    /// A [`f32`] value.
    F32 {
        /// Both the initial [`f32`] value, but also the default one
        /// in case of missing input.
        default: f32,
    },
    /// A [`f64`] value.
    F64 {
        /// Both the initial [`f64`] value, but also the default one
        /// in case of missing input.
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

/// Input data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InputData {
    /// Name.
    pub name: &'static str,
    /// Input structure.
    #[serde(rename = "structure")]
    pub structure: InputStructure,
}

impl core::cmp::PartialEq for InputData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl core::cmp::Eq for InputData {}

impl core::hash::Hash for InputData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl From<Input> for InputData {
    fn from(input: Input) -> Self {
        Self::new(input)
    }
}

impl InputData {
    fn new(input: Input) -> Self {
        Self {
            name: input.name,
            structure: input.structure,
        }
    }
}

/// A collection of [`InputData`]s.
pub type InputsData<const N: usize> = SerialCollection<InputData, N>;

/// All supported inputs.
#[derive(Debug, Clone, Copy)]
pub struct Input {
    // Name.
    name: &'static str,
    // Input structure.
    structure: InputStructure,
}

impl core::cmp::PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl core::cmp::Eq for Input {}

impl core::hash::Hash for Input {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Input {
    /// Returns [`Input`] name.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }

    /// Creates a [`bool`] input.
    #[must_use]
    #[inline]
    pub fn bool(name: &'static str, default: bool) -> Self {
        Self {
            name,
            structure: InputStructure::Bool { default },
        }
    }

    /// Creates an [`u8`] input.
    #[must_use]
    #[inline]
    pub fn u8(name: &'static str, default: u8) -> Self {
        Self {
            name,
            structure: InputStructure::U8 { default },
        }
    }

    /// Creates an [`u16`] input.
    #[must_use]
    #[inline]
    pub fn u16(name: &'static str, default: u16) -> Self {
        Self {
            name,
            structure: InputStructure::U16 { default },
        }
    }

    /// Creates an [`u32`] input.
    #[must_use]
    #[inline]
    pub fn u32(name: &'static str, default: u32) -> Self {
        Self {
            name,
            structure: InputStructure::U32 { default },
        }
    }

    /// Creates an [`u64`] input.
    #[must_use]
    #[inline]
    pub fn u64(name: &'static str, default: u64) -> Self {
        Self {
            name,
            structure: InputStructure::U64 { default },
        }
    }

    /// Creates a [`f32`] input.
    #[must_use]
    #[inline]
    pub fn f32(name: &'static str, default: f32) -> Self {
        Self {
            name,
            structure: InputStructure::F32 { default },
        }
    }

    /// Creates a [`f64`] input.
    #[must_use]
    #[inline]
    pub fn f64(name: &'static str, default: f64) -> Self {
        Self {
            name,
            structure: InputStructure::F64 { default },
        }
    }

    /// Creates an [`u64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64(name: &'static str, range: (u64, u64, u64)) -> Self {
        Self::rangeu64_with_default(name, range, 0)
    }

    /// Creates an [`u64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangeu64_with_default(name: &'static str, range: (u64, u64, u64), default: u64) -> Self {
        Self {
            name,
            structure: InputStructure::RangeU64 {
                min: range.0,
                max: range.1,
                step: range.2,
                default,
            },
        }
    }

    /// Creates an [`f64`] range without a default value.
    #[must_use]
    #[inline]
    pub fn rangef64(name: &'static str, range: (f64, f64, f64)) -> Self {
        Self::rangef64_with_default(name, range, 0.0)
    }

    /// Creates an [`f64`] range with a default value.
    #[must_use]
    #[inline]
    pub fn rangef64_with_default(name: &'static str, range: (f64, f64, f64), default: f64) -> Self {
        Self {
            name,
            structure: InputStructure::RangeF64 {
                min: range.0,
                max: range.1,
                step: range.2,
                default,
            },
        }
    }
}

/// A collection of [`Input`]s.
pub type Inputs<const N: usize> = Collection<Input, N>;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::{Input, InputData};

    #[test]
    fn test_values_inputs() {
        assert_eq!(
            serialize(InputData::from(Input::bool("bool", true))),
            json!({
                "name": "bool",
                "structure": {
                    "Bool": {
                        "default": true
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::u8("u8", 0))),
            json!({
                "name": "u8",
                "structure": {
                    "U8": {
                        "default": 0
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::u16("u16", 0))),
            json!({
                "name": "u16",
                "structure": {
                    "U16": {
                        "default": 0
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::u32("u32", 0))),
            json!({
                "name": "u32",
                "structure": {
                    "U32": {
                        "default": 0
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::u64("u64", 0))),
            json!({
                "name": "u64",
                "structure": {
                    "U64": {
                        "default": 0
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::f32("f32", 0.))),
            json!({
                "name": "f32",
                "structure": {
                    "F32": {
                        "default": 0.
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::f64("f64", 0.))),
            json!({
                "name": "f64",
                "structure": {
                    "F64": {
                        "default": 0.
                    }
                }
            })
        );
    }

    #[test]
    fn test_range_inputs() {
        assert_eq!(
            serialize(InputData::from(Input::rangeu64_with_default(
                "rangeu64",
                (0, 20, 1),
                5
            ))),
            json!({
                "name": "rangeu64",
                "structure": {
                    "RangeU64": {
                        "min": 0,
                        "max": 20,
                        "step": 1,
                        "default": 5,
                    }
                }
            })
        );

        assert_eq!(
            serialize(InputData::from(Input::rangef64_with_default(
                "rangef64",
                (0., 20., 0.1),
                5.
            ))),
            json!({
                "name": "rangef64",
                "structure": {
                    "RangeF64": {
                        "min": 0.,
                        "max": 20.,
                        "step": 0.1,
                        "default": 5.,
                    }
                }
            })
        );
    }
}
