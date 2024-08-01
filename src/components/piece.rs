use crate::{pieces::ChessPiece, Board, ChessSide, Space};

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

        if let Some(ref capture_rays) = self.piece.capture_rays {
            for ray in &self.piece.rays {
                let mut cast = board.check_iter(ray.cast(self.space.as_unchecked()));

                loop {
                    if let Some(space) = cast.next() {
                        if pieces.by_ref().any(|piece| piece.space == &space) {
                            break;
                        } else {
                            moves.push(space)
                        }
                    }
                }
            }

            for ray in capture_rays {
                let mut cast = board.check_iter(ray.cast(self.space.as_unchecked()));

                loop {
                    if let Some(space) = cast.next() {
                        if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                            if piece.side != self.side {
                                moves.push(space)
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            for ray in &self.piece.rays {
                let mut cast = board.check_iter(ray.cast(self.space.as_unchecked()));

                loop {
                    if let Some(space) = cast.next() {
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
            }
        }

        if self.piece.checkmate_possible {
            todo!()
        }

        moves
    }
}
