use crate::direction::ray::set::Builder as RaySetBuilder;
use crate::pieces::{standard, PrimitivePiece};

pub struct CommonKing;

impl PrimitivePiece for CommonKing {
    const VALUE: usize = 4;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(standard::King)
    }
}
