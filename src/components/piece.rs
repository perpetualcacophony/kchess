use crate::{game::AllPieces, pieces::PieceSet, Board, ChessSide, Space};

use crate::game;

bundle! {
    Piece:
    moved: bool,
    space: Space,
    piece: crate::PieceData,
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

        for (ray, cast) in self.piece.rays.cast(&self) {
            let mut cast = board.check_iter(cast);

            loop {
                if let Some(space) = cast.next() {
                    if let Some(piece) = pieces.by_ref().find(|piece| piece.space == &space) {
                        if piece.side != self.side && ray.capture() {
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
