use crate::direction::ray::RaySet;

use super::{Bishop, PieceKind, Rook};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Queen;

impl PieceKind for Queen {
    const VALUE: usize = 9;

    fn add_rays<'rays>(&self, set: &'rays mut RaySet) -> &'rays mut RaySet {
        set.add_piece(Bishop).add_piece(Rook)
    }
}
