use crate::pieces::PieceSet;

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

impl<'a, I, Set: PieceSet + 'a> Iterator for AllPieces<I>
where
    I: Iterator<Item = &'a Piece<Set>>,
{
    type Item = &'a Piece<Set>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, I, Set: PieceSet + 'a> AllPieces<I>
where
    Self: Iterator<Item = &'a Piece<Set>>,
{
    pub fn not_captured(self) -> impl Iterator<Item = &'a Piece<Set>> {
        self.filter(|piece| !piece.captured())
    }
}
