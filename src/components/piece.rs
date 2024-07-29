use crate::{
    direction::ray::{self, Ray},
    pieces, Board, ChessPiece, ChessSide, Space,
};

use crate::game;

bundle! {
    Piece
    moved: bool,
    space: Space,
    piece: ChessPiece,
    side: ChessSide,
    captured: bool
}

impl<'c> Piece<'c> {
    pub fn legal_moves(&self, board: &Board, pieces: game::AllPieces<'c>) -> Vec<Space> {
        let mut moves = Vec::new();
        let mut pieces = pieces.not_captured();

        match self.piece {
            ChessPiece::Pawn => {
                let mut steps =
                    board.check_iter(pieces::pawn::steps(*self.side, self.space.as_unchecked()));

                if *self.moved {
                    if let Some(space) = steps.next() {
                        if !pieces.by_ref().any(|piece| piece.space == &space) {
                            moves.push(space)
                        }
                    }
                } else {
                    for space in steps {
                        if !pieces.by_ref().any(|piece| piece.space == &space) {
                            moves.push(space)
                        }
                    }
                }

                for space in board.check_iter(pieces::pawn::captures(
                    *self.side,
                    self.space.as_unchecked(),
                )) {
                    if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        } else {
                            break;
                        }
                    } else {
                        moves.push(space)
                    }
                }
            }
            ChessPiece::Knight | ChessPiece::Bishop | ChessPiece::Rook | ChessPiece::Queen => {
                let rays: Vec<ray::IntoIter> = match self.piece {
                    ChessPiece::Knight => pieces::knight::rays()
                        .map(|ray| ray.cast(self.space.as_unchecked()))
                        .into(),
                    ChessPiece::Bishop => pieces::bishop::rays()
                        .map(|ray| ray.cast(self.space.as_unchecked()))
                        .into(),
                    ChessPiece::Rook => pieces::rook::rays()
                        .map(|ray| ray.cast(self.space.as_unchecked()))
                        .into(),
                    ChessPiece::Queen => pieces::queen::rays()
                        .map(|ray| ray.cast(self.space.as_unchecked()))
                        .into(),
                    _ => unreachable!(),
                };

                for ray in rays {
                    let mut ray = board.check_iter(ray);

                    loop {
                        if let Some(space) = ray.next() {
                            if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space)
                            {
                                if piece.side != self.side {
                                    moves.push(space)
                                } else {
                                    break;
                                }
                            } else {
                                moves.push(space)
                            }
                        }
                    }
                }
            }
            ChessPiece::King => {
                for space in board.check_iter(pieces::king::moves(self.space.as_unchecked())) {
                    if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        } else {
                            break;
                        }
                    } else {
                        moves.push(space)
                    }
                }

                todo!()
            }
        }

        moves
    }
}
