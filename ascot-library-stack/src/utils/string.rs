use core::str::FromStr;

use heapless::String as OtherString;

use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind, Result};

impl<const N: usize> core::fmt::Write for String<N> {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        self.push(s).map_err(|_| core::fmt::Error)
    }

    fn write_char(&mut self, c: char) -> core::result::Result<(), core::fmt::Error> {
        self.push_char(c).map_err(|_| core::fmt::Error)
    }
}

impl<const N: usize> String<N> {
    /// Creates an empty [`String`] with a fixed capacity of `N` bytes.
    #[must_use]
    pub const fn empty() -> Self {
        Self(OtherString::<N>::new())
    }

    /// Creates a [`String`].
    ///
    /// # Errors
    ///
    /// If the input text is greater than the `N` bytes, an error is returned.
    pub fn new(text: &str) -> Result<Self> {
        Ok(Self(OtherString::from_str(text).map_err(|()| {
            Error::new(
                ErrorKind::FixedText,
                "Impossible to create a new stack string.
Characters might not be UTF-8 or its length is wrong.",
            )
        })?))
    }

    /// Creates an infallible [`String`].
    ///
    /// If an error occurs, an empty [`String`] is returned.
    #[must_use]
    pub fn infallible(text: &str) -> Self {
        Self::new(text).unwrap_or(Self::empty())
    }

    /// Checks whether a [`String`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the associated string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Adds a string slice to [`String`].
    ///
    /// # Errors
    ///
    /// If the input text is greater than `N` bytes, an error is returned.
    pub fn push(&mut self, text: &str) -> Result<()> {
        self.0.push_str(text).map_err(|()| {
            Error::new(
                ErrorKind::FixedText,
                "Impossible to add another stack string at the end of the current one.",
            )
        })?;
        Ok(())
    }

    /// Adds a character to [`String`].
    ///
    /// # Errors
    ///
    /// If the input character causes the [`String`] to go beyond `N` bytes,
    /// an error is returned.
    pub fn push_char(&mut self, c: char) -> Result<()> {
        self.0.push(c).map_err(|()| {
            Error::new(
                ErrorKind::FixedText,
                "Impossible to add a char at the end of the stack string.",
            )
        })?;
        Ok(())
    }
}

/// A fixed-capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct String<const N: usize>(OtherString<N>);
