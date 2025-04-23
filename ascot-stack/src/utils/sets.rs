use core::hash::Hash;

use heapless::{FnvIndexSet, IndexSetIter};

use serde::{Deserialize, Serialize};

// Creates a `set` abstraction structure.
//
// Inputs:
//
// - Structure name
// - Types of values
// - Function argument name for a single value
// - Function argument name for a sequence of values
macro_rules! create_set {
    ($name:ident, $ty:ty, $arg:tt, $args:tt) => {
        #[doc = concat!("A fixed-length sequence of [`", stringify!($name), "`]s.")]
        #[derive(Debug, Clone, PartialEq, serde::Serialize)]
        pub struct $name<const N: usize>(heapless::FnvIndexSet<$ty, N>);

        impl<const N: usize> $name<N> {
            #[doc = concat!("Checks whether [`", stringify!($name), "`] is empty.")]
            #[must_use]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub(crate) const fn new() -> Self {
                Self(heapless::FnvIndexSet::new())
            }

            #[inline]
            fn insert(mut self, $arg: $ty) -> Self {
                let _ = self.0.insert($arg);
                self
            }
        }

        impl $name<2> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with one [`", stringify!($ty), "`].")]
            #[inline]
            #[must_use]
            pub fn one($arg: $ty) -> Self {
                Self::new().insert($arg)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with two [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn two($args: ($ty, $ty)) -> Self {
                Self::one($args.0).insert($args.1)
            }
        }

        impl $name<4> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with three [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn three($args: ($ty, $ty, $ty)) -> Self {
                Self::new().insert($args.0).insert($args.1).insert($args.2)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with four [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn four($args: ($ty, $ty, $ty, $ty)) -> Self {
                Self::three(($args.0, $args.1, $args.2)).insert($args.3)
            }
        }

        impl $name<8> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with five [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn five($args: ($ty, $ty, $ty, $ty, $ty)) -> Self {
                Self::new()
                    .insert($args.0)
                    .insert($args.1)
                    .insert($args.2)
                    .insert($args.3)
                    .insert($args.4)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with six [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn six($args: ($ty, $ty, $ty, $ty, $ty, $ty)) -> Self {
                Self::five(($args.0, $args.1, $args.2, $args.3, $args.4)).insert($args.5)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with seven [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn seven($args: ($ty, $ty, $ty, $ty, $ty, $ty, $ty)) -> Self {
                Self::six(($args.0, $args.1, $args.2, $args.3, $args.4, $args.5)).insert($args.6)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with eight [`", stringify!($ty), "`]s.")]
            #[inline]
            #[must_use]
            pub fn eight($args: ($ty, $ty, $ty, $ty, $ty, $ty, $ty, $ty)) -> Self {
                Self::seven((
                    $args.0, $args.1, $args.2, $args.3, $args.4, $args.5, $args.6,
                ))
                .insert($args.7)
            }
        }
    };
}

pub(crate) use create_set;

/// A set of elements for internal storage.
#[derive(Debug, Clone)]
pub struct Set<V: Eq + Hash, const N: usize>(FnvIndexSet<V, N>);

/// A serializable set of elements.
#[derive(Debug, Clone, Serialize)]
pub struct SerialSet<V: Eq + Hash, const N: usize>(FnvIndexSet<V, N>);

/// A serializable and deserializable set of elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputSet<V: Eq + Hash, const N: usize>(FnvIndexSet<V, N>);

macro_rules! from_set {
    ($for:ident) => {
        impl<V, V1, const N: usize> From<Set<V1, N>> for $for<V, N>
        where
            V: Clone + Copy + Eq + Hash + From<V1>,
            V1: Clone + Copy + Eq + Hash,
        {
            fn from(set: Set<V1, N>) -> Self {
                let mut new_set = Self::new();
                for element in set.iter() {
                    let _ = new_set.0.insert(V::from(*element));
                }
                new_set
            }
        }
    };
}

macro_rules! set_implementation {
    ($impl:ident $(,$trait:ident)?) => {
        impl<'a, V, const N: usize> IntoIterator for &'a $impl<V, N>
        where
            V: Clone + Copy + Eq + Hash,
        {
            type Item = &'a V;
            type IntoIter = IndexSetIter<'a, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<V, const N: usize> Default for $impl<V, N>
        where
            V: Clone + Copy + Eq + Hash,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<V, const N: usize> $impl<V, N>
        where
            V: Clone + Eq + Hash,
        {
            #[doc = concat!("Creates a [`", stringify!($impl), "`].")]
            #[must_use]
            pub const fn new() -> Self {
                Self(FnvIndexSet::new())
            }

            #[doc = concat!("Initializes a [`", stringify!($impl), "`] with a determined element.")]
            #[must_use]
            #[inline]
            pub fn init(element: V) -> Self {
                let mut elements = Self::new();
                elements.add(element);
                elements
            }

            #[doc = concat!("Inserts an element to a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn insert(mut self, element: V) -> Self {
                let _ = self.0.insert(element);
                self
            }

            #[doc = concat!("Adds an element to a [`", stringify!($impl), "`].")]
            #[inline]
            pub fn add(&mut self, element: V) {
                let _ = self.0.insert(element);
            }

            #[doc = concat!("Checks whether the [`", stringify!($impl), "`] is empty.")]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[doc = concat!("Returns the [`", stringify!($impl), "`] length.")]
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            #[doc = concat!("Checks whether the [`", stringify!($impl), "`] contains the given element.")]
            #[inline]
            pub fn contains(&self, element: &V) -> bool {
                self.0.contains(element)
            }

            #[doc = concat!("Returns an iterator over the [`", stringify!($impl), "`].")]
            #[doc = ""]
            #[doc = "**It iterates in the insertion order.**"]
            #[inline]
            pub fn iter(&self) -> IndexSetIter<'_, V> {
                self.0.iter()
            }
        }
    };
}

// Set implementation.
set_implementation!(Set);

// Serial set implementation.
set_implementation!(SerialSet);

// Convert from a set into a serial collection.
from_set!(SerialSet);
