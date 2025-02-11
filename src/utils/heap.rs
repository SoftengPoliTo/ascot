use core::hash::Hash;

use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use indexmap::Equivalent;

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
            type IntoIter = indexmap::set::IntoIter<V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, V> IntoIterator for &'a $impl<V>
        where
            V: Clone + Eq + Hash,
        {
            type Item = &'a V;
            type IntoIter = indexmap::set::Iter<'a, V>;

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
            pub fn iter(&self) -> indexmap::set::Iter<'_, V> {
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
set_implementation!(Set);

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
pub struct Map<K: Eq + Hash, V>(IndexMap<K, V>);

/// A serializable map of elements.
#[derive(Debug, Clone, Serialize)]
pub struct SerialMap<K: Eq + Hash, V>(IndexMap<K, V>);

/// A serializable and deserializable map of elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputMap<K: Eq + Hash, V>(IndexMap<K, V>);

macro_rules! from_map {
    ($for:ident) => {
        impl<K, V, K1, V1> From<Map<K1, V1>> for $for<K, V>
        where
            K: Clone + Eq + Hash + From<K1>,
            V: Clone + From<V1>,
            K1: Clone + Eq + Hash,
            V1: Clone,
        {
            fn from(map: Map<K1, V1>) -> Self {
                let mut new_map = Self::new();
                for (key, value) in map.iter() {
                    new_map
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
        impl<K, V> IntoIterator for $impl<K, V>
        where
            K: Clone + Eq + Hash,
        {
            type Item = (K, V);
            type IntoIter = indexmap::map::IntoIter<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, K, V> IntoIterator for &'a $impl<K, V>
        where
            K: Clone + Eq + Hash,
            V: Clone
        {
            type Item = (&'a K, &'a V);
            type IntoIter = indexmap::map::Iter<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<K, V> Default for $impl<K, V>
        where
            K: Clone + Eq + Hash,
            V: Clone
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<K, V> $impl<K, V>
        where
            K: Clone + Eq + Hash,
            V: Clone
        {
            #[doc = concat!("Creates a [`", stringify!($impl), "`].")]
            #[must_use]
            #[inline]
            pub fn new() -> Self {
                Self(IndexMap::new())
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
                self.0.insert(key, value);
                self
            }

            #[doc = concat!("Adds an element to a [`", stringify!($impl), "`].")]
            #[inline]
            pub fn add(&mut self, key: K, value: V) {
                self.0.insert(key, value);
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
            pub fn contains_key<Q>(&self, key: &Q) -> bool
            where
                Q: ?Sized + Hash + Equivalent<K>,
            {
                self.0.contains_key(key)
            }

            #[doc = concat!("Gets a value with the given key from [`", stringify!($impl), "`].")]
            #[inline]
            pub fn get<Q>(&self, key: &Q) -> Option<&V>
            where
                Q: ?Sized + Hash + Equivalent<K>,
            {
                self.0.get(key)
            }

            #[doc = concat!("Returns an iterator over the [`", stringify!($impl), "`].")]
            #[doc = ""]
            #[doc = "**It iterates in the insertion order.**"]
            #[must_use]
            #[inline]
            pub fn iter(&self) -> indexmap::map::Iter<'_, K, V> {
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
