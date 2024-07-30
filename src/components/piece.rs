use crate::{pieces::ChessPieceStruct, Board, ChessSide, Space};

use crate::game;

bundle! {
    Piece
    moved: bool,
    space: Space,
    piece: ChessPieceStruct,
    side: ChessSide,
    captured: bool
}

impl<'c> Piece<'c> {
    pub fn legal_moves(&self, board: &Board, pieces: game::AllPieces<'c>) -> Vec<Space> {
        let mut moves = Vec::new();
        let mut pieces = pieces.not_captured();

        let rays: Vec<_> = self
            .piece
            .rays
            .iter()
            .map(|ray| ray.cast(self.space.as_unchecked()))
            .collect();

        if let Some(ref capture_rays) = self.piece.capture_rays {
            let capture_rays: Vec<_> = capture_rays
                .iter()
                .map(|ray| ray.cast(self.space.as_unchecked()))
                .collect();

            for ray in rays {
                let mut ray = board.check_iter(ray);

                loop {
                    if let Some(space) = ray.next() {
                        if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                            break;
                        } else {
                            moves.push(space)
                        }
                    }
                }
            }

            for ray in capture_rays {
                let mut ray = board.check_iter(ray);

                loop {
                    if let Some(space) = ray.next() {
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
            for ray in rays {
                let mut ray = board.check_iter(ray);

                loop {
                    if let Some(space) = ray.next() {
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
