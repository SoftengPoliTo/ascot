use core::hash::Hash;

use heapless::{FnvIndexMap, FnvIndexSet};

use serde::{Deserialize, Serialize};

/// A set of elements for internal storage.
#[derive(Debug, PartialEq, Clone)]
pub struct Set<V: PartialEq + Eq + Hash>(FnvIndexSet<V, 8>);

/// A serializable set of elements.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SerialSet<V: PartialEq + Eq + Hash>(FnvIndexSet<V, 8>);

/// A serializable and deserializable set of elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputSet<V: PartialEq + Eq + Hash>(FnvIndexSet<V, 8>);

macro_rules! from_set {
    ($for:ident) => {
        impl<V, V1> From<Set<V1>> for $for<V>
        where
            V: Clone + Copy + PartialEq + Eq + Hash + From<V1>,
            V1: Clone + Copy + PartialEq + Eq + Hash,
        {
            fn from(set: Set<V1>) -> Self {
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
        impl<'a, V> IntoIterator for &'a $impl<V>
        where
            V: Clone + $($trait +)? PartialEq + Eq + Hash,
        {
            type Item = &'a V;
            type IntoIter = heapless::IndexSetIter<'a, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<V> Default for $impl<V>
        where
            V: Clone + $($trait +)? PartialEq + Eq + Hash,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<V> $impl<V>
        where
            V: Clone + $($trait +)? PartialEq + Eq + Hash,
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
            pub fn iter(&self) -> heapless::IndexSetIter<'_, V> {
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
        }
    };
}

// Set implementation.
set_implementation!(Set, Copy);

// Serial set implementation.
set_implementation!(SerialSet);

// Output set implementation.
set_implementation!(OutputSet);

// Convert from a set into a serial collection.
from_set!(SerialSet);
// Convert from a set into an output set.
from_set!(OutputSet);

/// A map of elements for internal storage.
#[derive(Debug, Clone)]
pub struct Map<K: PartialEq + Eq + Hash, V>(FnvIndexMap<K, V, 8>);

/// A serializable map of elements.
#[derive(Debug, Clone, Serialize)]
pub struct SerialMap<K: PartialEq + Eq + Hash, V>(FnvIndexMap<K, V, 8>);

/// A serializable and deserializable map of elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMap<K: PartialEq + Eq + Hash, V>(FnvIndexMap<K, V, 8>);

macro_rules! from_map {
    ($for:ident) => {
        impl<K, V, K1, V1> From<Map<K1, V1>> for $for<K, V>
        where
            K: Clone + Copy + PartialEq + Eq + Hash + From<K1>,
            V: Clone + Copy + PartialEq + Eq + From<V1>,
            K1: Clone + Copy + PartialEq + Eq + Hash,
            V1: Clone + Copy + PartialEq + Eq,
        {
            fn from(map: Map<K1, V1>) -> Self {
                let mut new_map = Self::new();
                for (key, value) in map.iter() {
                    let _ = new_map
                        .0
                        .insert(K::from(key.clone()), V::from(value.clone()));
                }
                new_map
            }
        }
    };
}

macro_rules! map_implementation {
    ($impl:ident) => {
        impl<'a, K, V> IntoIterator for &'a $impl<K, V>
        where
            K: Clone + PartialEq + Eq + Hash,
            V: Clone
        {
            type Item = (&'a K, &'a V);
            type IntoIter = heapless::IndexMapIter<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<K, V> Default for $impl<K, V>
        where
            K: Clone + PartialEq + Eq + Hash,
            V: Clone
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<K, V> $impl<K, V>
        where
            K: Clone + PartialEq + Eq + Hash,
            V: Clone
        {
            #[doc = concat!("Creates a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn new() -> Self {
                Self(FnvIndexMap::new())
            }

            #[doc = concat!("Initializes a [`", stringify!($impl), "`] with a determined element.")]
            #[must_use]
            #[inline]
            pub fn init(key: K, value: V) -> Self {
                Self::new().insert(key, value)
            }

            #[doc = concat!("Inserts an element to a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn insert(mut self, key: K, value: V) -> Self {
                let _ = self.0.insert(key, value);
                self
            }

            #[doc = concat!("Adds an element to a [`", stringify!($impl), "`].")]
            #[inline]
            pub fn add(&mut self, key: K, value: V) {
                let _ = self.0.insert(key, value);
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

            #[doc = concat!("Checks whether the [`", stringify!($impl), "`] contains the given key.")]
            #[inline]
            pub fn contains_key(&self, key: &K) -> bool {
                self.0.contains_key(key)
            }

            #[doc = concat!("Gets a value with the given key from [`", stringify!($impl), "`].")]
            #[inline]
            pub fn get(&self, key: &K) -> Option<&V> {
                self.0.get(key)
            }

            #[doc = concat!("Returns an iterator over the [`", stringify!($impl), "`].")]
            #[doc = ""]
            #[doc = "**It iterates in the insertion order.**"]
            #[must_use]
            #[inline]
            pub fn iter(&self) -> heapless::IndexMapIter<'_, K, V> {
                self.0.iter()
            }

            #[doc = concat!("Initializes [`", stringify!($impl), "`] with a list of `(key, value)`.")]
            #[inline]
            pub fn init_with_elements(input_elements: &[(K, V)]) -> Self {
                let mut elements = Self::new();
                for (key, value) in input_elements.iter() {
                    elements.add(key.clone(), value.clone());
                }
                elements
            }
        }
    };
}

// Map implementation.
map_implementation!(Map);

// Serial map implementation.
map_implementation!(SerialMap);

// Output map implementation.
map_implementation!(OutputMap);

// Convert from map into serial map.
from_map!(SerialMap);
// Convert from map into output map.
from_map!(OutputMap);
