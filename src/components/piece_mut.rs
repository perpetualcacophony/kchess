use crate::{Board, ChessPiece, ChessSide, Space};

bundle! {
    mut PieceMut
    moved: bool,
    space: Space,
    piece: ChessPiece,
    side: ChessSide,
    captured: bool
}

impl<'c> PieceMut<'c> {
    pub fn capture(self) {}

    pub fn promote(self) {
        *self.piece = ChessPiece::Queen;
    }

    pub fn move_to(self, space: Space, board: &Board, mut pieces: impl Iterator<Item = Self>) {
        *self.moved = true;

        if let Some(piece) = pieces.find(|piece| *piece.space == space) {
            piece.capture()
        }

        *self.space = space;

        if *self.piece == ChessPiece::Pawn && board.last_rank(*self.side, self.space.rank()) {
            self.promote();
        }
    }
}
