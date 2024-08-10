use crate::{direction::ray, game::Context, pieces::ChessPiece, ChessSide, Space};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartialPiece {
    pub moved: bool,
    pub space: Space,
    pub side: ChessSide,
    pub captured: bool,
}

#[derive(Clone, Debug)]
pub struct Piece<'set> {
    pub piece: &'set ChessPiece,
    partial: PartialPiece,
}

impl Piece<'_> {
    pub fn rays(&self) -> &ray::Set {
        self.piece.rays()
    }

    pub fn partial(&self) -> &PartialPiece {
        &self.partial
    }

    pub fn side(&self) -> &ChessSide {
        &self.partial().side
    }

    pub fn captured(&self) -> bool {
        self.partial().captured
    }

    pub fn dangerous_spaces<'a: 'b, 'b, T>(
        &'a self,
        ctx: Context<'b, '_, T>,
    ) -> impl Iterator<Item = Space> + 'b {
        ctx.pieces()
            .clone()
            .filter_map(move |piece| {
                (piece.side() != self.side()).then_some(piece.reachable_spaces(ctx))
            })
            .flatten()
    }

    pub fn space(&self) -> &Space {
        &self.partial().space
    }

    pub fn reachable_spaces<'a, T>(
        &'a self,
        ctx: Context<'a, '_, T>,
    ) -> impl Iterator<Item = Space> + 'a {
        self.rays().cast(self.partial()).flat_map(move |cast| {
            let capture = cast.ray().capture();
            let cast = ctx.board().check_iter(cast);
            cast.take_while(move |space| {
                let mut pieces = ctx.pieces().not_captured();

                if let Some(piece) = pieces.find(|piece| piece.space() == space) {
                    piece.side() != self.side() && capture
                } else {
                    true
                }
            })
        })
    }
}
