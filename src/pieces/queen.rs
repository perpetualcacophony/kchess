use crate::direction::ray::RaySetBuilder;

use super::{Bishop, PieceKind, Rook};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Queen;

impl PieceKind for Queen {
    const VALUE: usize = 9;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(Bishop).add_piece(Rook)
    }
}
