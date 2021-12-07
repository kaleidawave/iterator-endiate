pub trait EndiateIteratorExt: Sized {
    fn endiate(self) -> Endiate<Self>;
}

impl<Iter: ExactSizeIterator> EndiateIteratorExt for Iter {
    /// Similar to [`Iterator::enumerate`] but rather including indexes, it includes whether the item
    /// is the last item in the iterator
    /// ```
    /// use iterator_endiate::EndiateIteratorExt;
    /// let endiated = vec![1, 2, 3].into_iter().endiate().collect::<Vec<_>>();
    /// assert_eq!(
    ///     endiated,
    ///     [
    ///         (false, 1),
    ///         (false, 2),
    ///         (true, 3),
    ///     ]
    /// )
    /// ```
    fn endiate(self) -> Endiate<Self> {
        Endiate { iterator: self }
    }
}

pub struct Endiate<Iter> {
    iterator: Iter,
}

impl<Iter: ExactSizeIterator> Iterator for Endiate<Iter> {
    type Item = (bool, Iter::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|v| (self.iterator.len() == 0, v))
    }
}
