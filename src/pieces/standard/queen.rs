use crate::direction::ray::set::Builder as RaySetBuilder;

use super::{Bishop, PrimitivePiece, Rook};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Queen;

impl PrimitivePiece for Queen {
    const VALUE: usize = 9;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_piece::<Bishop>().add_piece::<Rook>()
    }
}
