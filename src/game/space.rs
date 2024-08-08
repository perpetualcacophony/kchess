use crate::{game::Piece, Space};

use super::Context;

#[derive(Clone, Debug)]
pub struct SpaceContext<'ctx> {
    inner: Space,
    piece: Option<&'ctx Piece>,
    threats: Vec<&'ctx Piece>,
}

impl<'ctx> SpaceContext<'ctx> {
    pub(super) fn new(ctx: Context<'ctx>, space: Space) -> Self {
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

    pub fn piece(&self) -> Option<&Piece> {
        self.piece
    }
}

impl AsRef<Space> for SpaceContext<'_> {
    fn as_ref(&self) -> &Space {
        self.space()
    }
}
