/// A device state accessible through a route handler.
pub struct State<S>(pub S);

/// A trait used in conjunction with the [`State`] extractor to retrieve
/// specific parts of the firmware state.
pub trait ValueFromRef {
    /// Converts a self reference into a value.
    #[must_use]
    fn value_from_ref(&self) -> Self;
}

impl ValueFromRef for () {
    fn value_from_ref(&self) -> Self {}
}
