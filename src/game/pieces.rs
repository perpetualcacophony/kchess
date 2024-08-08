use super::Piece;

#[derive(Clone, Copy, Debug)]
pub struct AllPieces<I> {
    inner: I,
}

impl<I> AllPieces<I> {
    pub(super) fn new(iter: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            inner: iter.into_iter(),
        }
    }
}

impl<'a, 'set: 'a, I> Iterator for AllPieces<I>
where
    I: Iterator<Item = &'a Piece<'set>>,
{
    type Item = &'a Piece<'set>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, 'set: 'a, I> AllPieces<I>
where
    Self: Iterator<Item = &'a Piece<'set>>,
{
    pub fn not_captured(self) -> impl Iterator<Item = &'a Piece<'set>> {
        self.filter(|piece| !piece.captured())
    }
}
