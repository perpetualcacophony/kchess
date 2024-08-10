use super::{ChessPiece, PrimitivePiece};

#[derive(Debug)]
pub struct PieceSet {
    inner: Vec<ChessPiece>,
}

impl PieceSet {
    pub fn iter(&self) -> Iter {
        Iter::new(&self.inner)
    }

    pub fn promotions(&self) -> impl Iterator<Item = &ChessPiece> {
        self.iter().filter(|piece| piece.valid_promotion())
    }
}

impl<'a> IntoIterator for &'a PieceSet {
    type Item = &'a ChessPiece;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<ChessPiece> for PieceSet {
    fn from_iter<T: IntoIterator<Item = ChessPiece>>(iter: T) -> Self {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}

pub fn standard() -> PieceSet {
    PieceSet::from_iter([
        super::Pawn::chess_piece(),
        super::Knight::chess_piece(),
        super::Bishop::chess_piece(),
        super::Rook::chess_piece(),
        super::Queen::chess_piece(),
        super::King::chess_piece(),
    ])
}

pub struct Iter<'a> {
    inner: std::slice::Iter<'a, ChessPiece>,
}

impl<'a> Iter<'a> {
    fn new(iter: impl IntoIterator<IntoIter = std::slice::Iter<'a, ChessPiece>>) -> Self {
        Self {
            inner: iter.into_iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a ChessPiece;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
