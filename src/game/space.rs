use crate::{game::Piece, Space};

use super::Context;

#[derive(Clone, Debug)]
pub struct SpaceContext<'ctx, 'set> {
    inner: Space,
    piece: Option<&'ctx Piece<'set>>,
    threats: Vec<&'ctx Piece<'set>>,
}

impl<'ctx, 'set> SpaceContext<'ctx, 'set> {
    pub(super) fn new(ctx: Context<'ctx, 'set>, space: Space) -> Self {
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

    pub fn piece(&self) -> Option<&Piece<'set>> {
        self.piece
    }
}

impl AsRef<Space> for SpaceContext<'_, '_> {
    fn as_ref(&self) -> &Space {
        self.space()
    }
}
