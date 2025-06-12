use core::hash::Hash;

use indexmap::set::{IndexSet, IntoIter, Iter};

use serde::{Deserialize, Serialize};

/// A set of elements for internal storage.
#[derive(Debug, Clone)]
pub struct Set<V: Eq + Hash>(IndexSet<V>);

/// A serializable set of elements.
#[derive(Debug, Clone, Serialize)]
pub struct SerialSet<V: Eq + Hash>(IndexSet<V>);

/// A serializable and deserializable set of elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputSet<V: Eq + Hash>(IndexSet<V>);

macro_rules! from_set {
    ($for:ident) => {
        impl<V, V1> From<Set<V1>> for $for<V>
        where
            V: Clone + Eq + Hash + From<V1>,
            V1: Clone + Eq + Hash,
        {
            fn from(set: Set<V1>) -> Self {
                let mut new_set = Self::new();
                for element in set.iter() {
                    new_set.0.insert(V::from(element.clone()));
                }
                new_set
            }
        }
    };
}

macro_rules! set_implementation {
    ($impl:ident) => {
        impl<V> IntoIterator for $impl<V>
        where
            V: Clone + Eq + Hash,
        {
            type Item = V;
            type IntoIter = IntoIter<V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, V> IntoIterator for &'a $impl<V>
        where
            V: Clone + Eq + Hash,
        {
            type Item = &'a V;
            type IntoIter = Iter<'a, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<V> Default for $impl<V>
        where
            V: Clone + Eq + Hash,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<V> $impl<V>
        where
            V: Clone + Eq +  Hash,
        {
            #[doc = concat!("Creates a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn new() -> Self {
                Self(IndexSet::default())
            }

            #[doc = concat!("Initializes a [`", stringify!($impl), "`] with a determined element.")]
            #[must_use]
            #[inline]
            pub fn init(element: V) -> Self {
                Self::new().insert(element)
            }

            #[doc = concat!("Inserts an element to a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn insert(mut self, element: V) -> Self {
                self.0.insert(element);
                self
            }

            #[doc = concat!("Adds an element to a [`", stringify!($impl), "`].")]
            #[inline]
            pub fn add(&mut self, element: V) {
                self.0.insert(element);
            }

            #[doc = concat!("Checks whether the [`", stringify!($impl), "`] is empty.")]
            #[must_use]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[doc = concat!("Returns the [`", stringify!($impl), "`] length.")]
            #[must_use]
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
            #[must_use]
            #[inline]
            pub fn iter(&self) -> Iter<'_, V> {
                self.0.iter()
            }

            #[doc = concat!("Initializes [`", stringify!($impl), "`] with a list of elements.")]
            #[inline]
            pub fn init_with_elements(input_elements: &[V]) -> Self {
                let mut elements = Self::new();
                for element in input_elements.iter() {
                    elements.add(element.clone());
                }
                elements
            }

            #[doc = concat!("Merges all elements from another [`", stringify!($impl), "`] into this one.")]
            #[inline]
            pub fn merge(&mut self, element: &Self) {
                self.0 = self.0.union(&element.0).cloned().collect();
            }

            #[doc = concat!("Consumes and inserts all elements from another [`", stringify!($impl), "`] into this one.")]
            #[inline]
            pub fn extend(mut self, other: Self) -> Self {
                self.0.extend(other);
                self
            }
        }
    };
}

// Set implementation.
set_implementation!(Set);

// Serial set implementation.
set_implementation!(SerialSet);

// Output set implementation.
set_implementation!(OutputSet);

// Convert from a set into a serial collection.
from_set!(SerialSet);
// Convert from a set into an output set.
from_set!(OutputSet);
