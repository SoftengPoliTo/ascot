use serde::{Deserialize, Serialize};

use crate::collections::Collection;

#[cfg(feature = "alloc")]
mod private_input {
    use super::{Deserialize, Serialize};

    /// An [`Input`] structure.
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        /// This kind of input can contain an unknown or a precise sequence of
        /// characters expressed either as a fixed-size or an allocated string.
        CharsSequence {
            /// Initial characters sequence, which also represents the default
            /// value.
            default: alloc::borrow::Cow<'static, str>,
        },
    }

    /// Input data.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InputData {
        /// Name.
        pub name: alloc::borrow::Cow<'static, str>,
        /// Input structure.
        #[serde(rename = "structure")]
        pub structure: InputStructure,
    }

    /// All supported inputs.
    #[derive(Debug, Clone)]
    pub struct Input {
        // Name.
        pub(super) name: &'static str,
        // Input structure.
        pub(super) structure: InputStructure,
    }

    impl InputData {
        pub(super) fn new(input: Input) -> Self {
            Self {
                name: input.name.into(),
                structure: input.structure,
            }
        }
    }

    /// A collection of [`InputData`]s.
    pub type InputsData = crate::collections::OutputCollection<InputData>;
}

#[cfg(not(feature = "alloc"))]
mod private_input {
    use super::{Deserialize, Serialize};

    /// An [`Input`] structure.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum InputStructure {
        /// A [`bool`] value.
        Bool {
            /// Both the initial [`bool`] value, but also the default one in case of
            /// missing input.
            default: bool,
        },
        /// A [`u8`] value.
        U8 {
            /// Both the initial [`u8`] value, but also the default one in case of
            /// missing input.
            default: u8,
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
    #[derive(Debug, Clone, Copy, Serialize)]
    pub struct InputData {
        /// Name.
        pub name: &'static str,
        /// Input structure.
        #[serde(rename = "structure")]
        pub structure: InputStructure,
    }

    /// All supported inputs.
    #[derive(Debug, Clone, Copy)]
    pub struct Input {
        // Name.
        pub(super) name: &'static str,
        // Input structure.
        pub(super) structure: InputStructure,
    }

    impl InputData {
        pub(super) fn new(input: Input) -> Self {
            Self {
                name: input.name,
                structure: input.structure,
            }
        }
    }

    /// A collection of [`InputData`]s.
    pub type InputsData = crate::collections::SerialCollection<InputData>;
}

pub use private_input::{Input, InputData, InputStructure, InputsData};

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
    #[inline]
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

    /// Creates a characters sequence with a default value.
    #[must_use]
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn characters_sequence(
        name: &'static str,
        default: impl Into<alloc::borrow::Cow<'static, str>>,
    ) -> Self {
        Self {
            name,
            structure: InputStructure::CharsSequence {
                default: default.into(),
            },
        }
    }
}

/// A collection of [`Input`]s.
pub type Inputs = Collection<Input>;

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests {
    use crate::alloc::string::ToString;
    use crate::{deserialize, serialize};

    use super::{Input, InputData};

    #[test]
    fn test_all_inputs() {
        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::bool("bool", true)))),
            InputData::from(Input::bool("bool", true))
        );

        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::u8("u8", 0)))),
            InputData::from(Input::u8("u8", 0))
        );

        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::rangeu64_with_default(
                "rangeu64",
                (0, 20, 1),
                5
            )))),
            InputData::from(Input::rangeu64_with_default("rangeu64", (0, 20, 1), 5))
        );

        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::rangef64_with_default(
                "rangef64",
                (0., 20., 0.1),
                5.
            )))),
            InputData::from(Input::rangef64_with_default("rangef64", (0., 20., 0.1), 5.))
        );

        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::characters_sequence(
                "greeting", "hello",
            )))),
            InputData::from(Input::characters_sequence("greeting", "hello"))
        );

        assert_eq!(
            deserialize::<InputData>(serialize(InputData::from(Input::characters_sequence(
                "greeting",
                "hello".to_string(),
            )))),
            InputData::from(Input::characters_sequence("greeting", "hello"))
        );
    }
}

#[cfg(not(feature = "alloc"))]
#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize;

    use super::{Input, InputData};

    #[test]
    fn test_all_inputs() {
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
