use crate::{pieces::ChessPiece, Board, ChessSide, Space};

bundle! {
    mut PieceMut
    moved: bool,
    space: Space,
    piece: ChessPiece,
    side: ChessSide,
    captured: bool
}

impl<'c> PieceMut<'c> {
    pub fn capture(&mut self) {
        *self.captured = true;
    }

    pub fn promote(&mut self) {
        *self.piece = ChessPiece::queen();
    }

    pub fn move_to(
        &mut self,
        space: Space,
        board: &Board,
        mut pieces: impl Iterator<Item = &'c mut Self>,
    ) {
        *self.moved = true;

        if let Some(piece) = pieces.find(|piece| *piece.space == space) {
            piece.capture()
        }

        *self.space = space;

        if self.piece.can_promote && board.opposite_end(*self.side, *self.space) {
            self.promote();
        }
    }
}
