// Creates a `map` abstraction structure.
//
// Inputs:
//
// - Structure name
// - Types of (key, value) pairs
// - Function argument name for a single pair value
// - Function argument name for a sequence of pairs values
macro_rules! create_map {
    ($name:ident, ($key:ty, $value:ty), $arg:tt, $args:tt) => {
        #[doc = concat!("A fixed-length map of ([`", stringify!($name), "`]s.")]
        pub struct $name<const N: usize>(heapless::FnvIndexMap<$key, $value, N>);

        impl<const N: usize> $name<N> {
            #[doc = concat!("Checks whether [`", stringify!($name), "`] is empty.")]
            #[must_use]
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub(crate) const fn new() -> Self {
                Self(heapless::FnvIndexMap::new())
            }

            #[inline]
            fn insert(mut self, $arg: ($key, $value)) -> Self {
                let _ = self.0.insert($arg.0, $arg.1);
                self
            }
        }

        impl $name<2> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with one ([`", stringify!($key), "`],[`", stringify!($value), "`]) pair.")]
            #[inline]
            #[must_use]
            pub fn one($arg: ($key, $value)) -> Self {
                Self::new().insert($arg)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with two ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn two($args: (($key, $value), ($key, $value))) -> Self {
                Self::one($args.0).insert($args.1)
            }
        }

        impl $name<4> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with three ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn three($args: (($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::new().insert($args.0).insert($args.1).insert($args.2)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with four ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn four($args: (($key, $value), ($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::three(($args.0, $args.1, $args.2)).insert($args.3)
            }
        }

        impl $name<8> {
            #[doc = concat!("Creates [`", stringify!($name), "`] with five ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn five($args: (($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::new()
                    .insert($args.0)
                    .insert($args.1)
                    .insert($args.2)
                    .insert($args.3)
                    .insert($args.4)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with six ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn six($args: (($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::five(($args.0, $args.1, $args.2, $args.3, $args.4)).insert($args.5)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with seven ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn seven($args: (($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::six(($args.0, $args.1, $args.2, $args.3, $args.4, $args.5)).insert($args.6)
            }

            #[doc = concat!("Creates [`", stringify!($name), "`] with eight ([`", stringify!($key), "`],[`", stringify!($value), "`]) pairs.")]
            #[inline]
            #[must_use]
            pub fn eight($args: (($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value), ($key, $value))) -> Self {
                Self::seven((
                    $args.0, $args.1, $args.2, $args.3, $args.4, $args.5, $args.6,
                ))
                .insert($args.7)
            }
        }
    };
}

pub(crate) use create_map;
