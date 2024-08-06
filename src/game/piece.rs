use crate::{
    game::{space::SpaceContext, Context},
    ChessSide, Space,
};

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
            .filter_map(move |piece| {
                (piece.side != self.side).then_some(piece.reachable_spaces(ctx))
            })
            .flatten()
    }

    pub fn space<'ctx>(&self, ctx: Context<'ctx>) -> SpaceContext<'ctx> {
        SpaceContext::new(ctx, self.space)
    }

    pub fn reachable_spaces<'a>(&'a self, ctx: Context<'a>) -> impl Iterator<Item = Space> + 'a {
        self.data.rays.cast(self).flat_map(move |(ray, cast)| {
            let cast = ctx.board().check_iter(cast);
            cast.take_while(move |space| {
                let mut pieces = ctx.pieces().not_captured();

                if let Some(piece) = pieces.find(|piece| &piece.space == space) {
                    piece.side != self.side && ray.capture()
                } else {
                    true
                }
            })
        })
    }
}
