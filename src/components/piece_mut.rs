use crate::{pieces::ChessPieceStruct, Board, ChessPiece, ChessSide, Space};

bundle! {
    mut PieceMut
    moved: bool,
    space: Space,
    piece: ChessPieceStruct,
    side: ChessSide,
    captured: bool
}

impl<'c> PieceMut<'c> {
    pub fn capture(&mut self) {}

    pub fn promote(&mut self) {
        *self.piece = ChessPieceStruct::queen();
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

        if self.piece.can_promote && board.last_rank(*self.side, self.space.rank()) {
            self.promote();
        }
    }
}
