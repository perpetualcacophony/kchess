use crate::ChessSide;

use super::Piece;

pub struct Side<'c> {
    pub side: ChessSide,
    pub pieces: Vec<Piece<'c>>,
}

impl<'c> Side<'c> {
    pub fn material(&self) -> usize {
        self.active_pieces()
            .fold(0, |total, piece| total + piece.piece.data.value)
    }

    pub fn active_pieces(&self) -> impl Iterator<Item = &Piece<'c>> {
        self.pieces.iter().filter(|piece| !piece.captured)
    }

    pub fn advantage(&self, rhs: &Self) -> isize {
        self.material() as isize - rhs.material() as isize
    }
}
