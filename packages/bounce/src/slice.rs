pub trait Slice: PartialEq + Default + Clone {
    type Action;

    /// Performs a reduce action.
    ///
    /// This always yields a new instance of [`Slice`] so it can be compared with the previous
    /// slice with [`PartialEq`].
    fn reduce(&self, action: Self::Action) -> Self;
}
