use crate::{game::components::Piece, Space};

use super::Context;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpaceContext<'ctx> {
    inner: &'ctx Space,
    piece: Option<&'ctx Piece>,
}

impl<'ctx> SpaceContext<'ctx> {
    pub(super) fn new(ctx: Context<'ctx>, space: &'ctx Space) -> Self {
        Self {
            inner: space,
            piece: ctx.pieces().find(|piece| &piece.space == space),
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
