use crate::{game::Piece, pieces::PieceSet, Space};

use super::Context;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceContext<'ctx, Set: PieceSet> {
    inner: Space,
    piece: Option<&'ctx Piece<Set>>,
    threats: Vec<&'ctx Piece<Set>>,
}

impl<'ctx, Set: PieceSet> SpaceContext<'ctx, Set> {
    pub(super) fn new(ctx: Context<'ctx, Set>, space: Space) -> Self {
        Self {
            inner: space,
            piece: ctx.pieces().find(|piece| piece.space() == &space),
            threats: ctx
                .pieces()
                .filter(|piece| {
                    piece
                        .reachable_spaces(ctx)
                        .any(|reachable| reachable == space)
                })
                .collect(),
        }
    }

    pub fn space(&self) -> &Space {
        &self.inner
    }

    pub fn piece(&self) -> Option<&Piece<Set>> {
        self.piece
    }
}

impl<Set: PieceSet> AsRef<Space> for SpaceContext<'_, Set> {
    fn as_ref(&self) -> &Space {
        self.space()
    }
}
