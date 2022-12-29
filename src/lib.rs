#![doc = include_str!("../README.md")]

/// Extension trait for adding [EndiateIteratorExt::endiate] and [EndiateIteratorExt::nendiate]
pub trait EndiateIteratorExt: Sized {
    /// Similar to [`Iterator::enumerate`] but rather including indexes, it includes whether the item
    /// is the last item in the iterator
    /// ```
    /// use iterator_endiate::EndiateIteratorExt;
    /// let endiate = [1, 2, 3].into_iter().endiate().collect::<Vec<_>>();
    /// assert_eq!(
    ///     endiate,
    ///     [
    ///         (false, 1),
    ///         (false, 2),
    ///         (true, 3),
    ///     ]
    /// )
    /// ```
    fn endiate(self) -> Endiate<Self>;

    /// Same as [EndiateIteratorExt::endiate] but bool is **not** at end
    /// ```
    /// use iterator_endiate::EndiateIteratorExt;
    /// let nendiate = [1, 2, 3].into_iter().nendiate().collect::<Vec<_>>();
    /// assert_eq!(
    ///     nendiate,
    ///     [
    ///         (true, 1),
    ///         (true, 2),
    ///         (false, 3),
    ///     ]
    /// )
    /// ```
    fn nendiate(self) -> NEndiate<Self>;
}

impl<Iter: ExactSizeIterator> EndiateIteratorExt for Iter {
    fn endiate(self) -> Endiate<Self> {
        Endiate(self)
    }

    fn nendiate(self) -> NEndiate<Self> {
        NEndiate(Endiate(self))
    }
}

/// From [EndiateIteratorExt::endiate]
pub struct Endiate<Iter>(pub(crate) Iter);

/// From [EndiateIteratorExt::nendiate]
pub struct NEndiate<Iter>(pub(crate) Endiate<Iter>);

impl<Iter: ExactSizeIterator> Iterator for Endiate<Iter> {
    type Item = (bool, Iter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| (self.0.len() == 0, v))
    }
}

impl<Iter: ExactSizeIterator> Iterator for NEndiate<Iter> {
    type Item = (bool, Iter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(at_end, v)| (!at_end, v))
    }
}

impl<Iter: ExactSizeIterator> ExactSizeIterator for Endiate<Iter> {}

impl<Iter: ExactSizeIterator> ExactSizeIterator for NEndiate<Iter> {}
