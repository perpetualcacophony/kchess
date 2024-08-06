use crate::{pieces::PieceSet, ChessSide};

use super::Piece;

pub struct Side<'ctx, Set: PieceSet> {
    pub side: ChessSide,
    pub pieces: Vec<&'ctx Piece<Set>>,
}

impl<'c, Set: PieceSet> Side<'c, Set> {
    pub fn material(&self) -> usize {
        self.active_pieces()
            .fold(0, |total, piece| total + piece.piece.stats.value)
    }

    pub fn active_pieces(&self) -> impl Iterator<Item = &Piece<Set>> {
        self.pieces
            .iter()
            .filter(|piece| !piece.captured())
            .copied()
    }

    pub fn advantage(&self, rhs: &Self) -> isize {
        self.material() as isize - rhs.material() as isize
    }
}
