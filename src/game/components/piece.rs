use crate::{
    game::{pieces::AllPieces, Context},
    Board, ChessSide, Space,
};

use crate::game;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub moved: bool,
    pub space: Space,
    pub data: crate::PieceData,
    pub side: ChessSide,
    pub captured: bool,
}

impl Piece {
    pub fn dangerous_spaces<'a: 'b, 'b>(
        &'a self,
        ctx: Context<'b>,
    ) -> impl Iterator<Item = Space> + 'b {
        ctx.pieces()
            .clone()
            .filter_map(move |piece| (piece.side != self.side).then_some(piece.legal_moves(ctx)))
            .flatten()
            .map(|move_to| move_to.space)
    }

    fn legal_moves<'a, 'b>(&'a self, ctx: Context<'b>) -> Vec<MoveTo<'a, 'b>> {
        let mut moves = Vec::new();

        let unfiltered = ctx.pieces();

        let mut pieces = unfiltered.clone().not_captured();

        for (ray, cast) in self.data.rays.cast(&self) {
            let mut cast = ctx.board().check_iter(cast);

            loop {
                if let Some(space) = cast.next() {
                    if let Some(piece) = pieces.by_ref().find(|piece| piece.space == space) {
                        if piece.side != self.side && ray.capture() {
                            moves.push(MoveTo {
                                piece: self,
                                space,
                                capture: Some(piece),
                            })
                        } else {
                            break;
                        }
                    } else {
                        moves.push(MoveTo {
                            piece: self,
                            space,
                            capture: None,
                        })
                    }
                }
            }
        }

        if self.data.checkmate_possible {
            let dangerous_spaces = self.dangerous_spaces(ctx).collect::<Vec<_>>();
            moves.retain(|move_to| !dangerous_spaces.contains(&move_to.space));
        }

        moves
    }
}

pub struct MoveTo<'a, 'b> {
    piece: &'a Piece,
    space: Space,
    capture: Option<&'b Piece>,
}
