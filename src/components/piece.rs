use crate::{game::AllPieces, pieces::PieceData, Board, ChessSide, Space};

use crate::game;

bundle! {
    Piece
    moved: bool,
    space: Space,
    piece: PieceData,
    side: ChessSide,
    captured: bool
}

impl<'c> Piece<'c> {
    pub fn dangerous_spaces(
        self,
        board: &'c Board,
        all_pieces: AllPieces<'c>,
    ) -> impl Iterator<Item = Space> + 'c {
        all_pieces
            .clone()
            .filter_map(move |piece| {
                (piece.side != self.side)
                    .then_some(piece.legal_moves_inner(board, all_pieces.clone()))
            })
            .flatten()
    }

    fn legal_moves_inner(self, board: &'c Board, pieces: AllPieces<'c>) -> Vec<Space> {
        let mut moves = Vec::new();

        let unfiltered = pieces.into_iter();

        let mut pieces = unfiltered.clone().filter(|piece| !piece.captured);

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
            let dangerous_spaces = self.dangerous_spaces(board, unfiltered).collect::<Vec<_>>();
            moves.retain(|space| !dangerous_spaces.contains(space));
        }

        moves
    }

    pub fn legal_moves(self, board: &Board, pieces: game::AllPieces<'c>) -> Vec<Space> {
        self.legal_moves_inner(board, pieces)
    }
}
