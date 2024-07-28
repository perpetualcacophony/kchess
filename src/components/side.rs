use crate::{ChessPiece, ChessSide};

use super::Piece;

pub struct Side<'c> {
    pub side: ChessSide,
    pub pieces: Vec<Piece<'c>>,
}

impl<'c> Side<'c> {
    pub fn material(&self) -> usize {
        self.active_pieces()
            .fold(0, |total, piece| total + piece.piece.value())
    }

    pub fn active_pieces(&self) -> impl Iterator<Item = &Piece<'c>> {
        self.pieces.iter().filter(|piece| !piece.captured)
    }

    pub fn advantage(&self, rhs: &Self) -> isize {
        self.material() as isize - rhs.material() as isize
    }

    pub fn pieces(&self, kind: ChessPiece) -> impl Iterator<Item = &Piece<'c>> {
        self.active_pieces()
            .filter(move |piece| piece.piece == &kind)
    }
}
