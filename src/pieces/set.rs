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
    pawn: ChessPiece,
    knight: ChessPiece,
    bishop: ChessPiece,
    rook: ChessPiece,
    queen: ChessPiece,
    king: ChessPiece,
}

impl Default for StandardSet {
    fn default() -> Self {
        Self {
            pawn: super::Pawn::chess_piece(),
            knight: super::Knight::chess_piece(),
            bishop: super::Bishop::chess_piece(),
            rook: super::Rook::chess_piece(),
            queen: super::Queen::chess_piece(),
            king: super::King::chess_piece(),
        }
    }
}

impl PieceSet for StandardSet {
    fn pieces(&self) -> impl IntoIterator<Item = &ChessPiece> {
        [
            &self.pawn,
            &self.knight,
            &self.bishop,
            &self.rook,
            &self.queen,
            &self.king,
        ]
    }

    fn default_promotion(&self) -> &ChessPiece {
        &self.queen
    }
}
