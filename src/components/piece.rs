use crate::{pieces, Board, ChessPiece, ChessSide, Space};

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
    pub fn legal_moves(&self, board: &Board, mut pieces: game::Pieces<'c>) -> Vec<Space> {
        let mut moves = Vec::new();

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
            ChessPiece::Knight => {
                for space in board.check_iter(pieces::knight::moves(self.space.as_unchecked())) {
                    if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        }
                    } else {
                        moves.push(space)
                    }
                }
            }

            ChessPiece::Bishop | ChessPiece::Rook | ChessPiece::Queen => {
                let rays: Vec<Box<dyn Iterator<Item = Space> + '_>> = match self.piece {
                    ChessPiece::Bishop => pieces::bishop::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    ChessPiece::Rook => pieces::rook::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    ChessPiece::Queen => pieces::queen::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    _ => unreachable!(),
                };

                for mut ray in rays {
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
