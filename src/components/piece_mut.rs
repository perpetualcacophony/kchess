use crate::{
    pieces::{Piece, PieceData, PieceSet, Queen},
    Board, ChessSide, Space,
};

bundle! {
    mut PieceMut:
    moved: bool,
    space: Space,
    piece: crate::pieces::Piece<P>,
    side: ChessSide,
    captured: bool
}

impl<'c, P: PieceSet + Copy> PieceMut<'c, P> {
    pub fn capture(&mut self) {
        *self.captured = true;
    }

    pub fn promote(&mut self, into: P) {
        *self.piece = Piece::new(into);
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

        if self.piece.data.can_promote && board.opposite_end(*self.side, *self.space) {
            self.promote(P::promotions().last().copied().unwrap());
        }
    }
}
