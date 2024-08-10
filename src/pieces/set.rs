use super::{ChessPiece, PrimitivePiece};

pub trait PieceSet: Default {
    fn pieces(&self) -> impl IntoIterator<Item = &ChessPiece>;

    fn promotions(&self) -> impl Iterator<Item = &ChessPiece> {
        self.pieces()
            .into_iter()
            .filter(|piece| piece.valid_promotion())
    }

    fn default_promotion(&self) -> &ChessPiece;
}

pub struct StandardSet {
    array: [ChessPiece; 6],
    queen: usize,
}

impl Default for StandardSet {
    fn default() -> Self {
        Self {
            array: [
                super::Pawn::chess_piece(),
                super::Knight::chess_piece(),
                super::Bishop::chess_piece(),
                super::Rook::chess_piece(),
                super::Queen::chess_piece(),
                super::King::chess_piece(),
            ],
            queen: 4,
        }
    }
}

impl PieceSet for StandardSet {
    fn pieces(&self) -> impl IntoIterator<Item = &ChessPiece> {
        self.array.as_slice()
    }

    fn default_promotion(&self) -> &ChessPiece {
        &self.array[self.queen]
    }
}
