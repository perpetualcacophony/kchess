use crate::direction::ray::set::Builder as RaySetBuilder;
use crate::pieces::{standard, PrimitivePiece};

pub struct Knook;

impl PrimitivePiece for Knook {
    const VALUE: usize = 8;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(standard::Knight).add_piece(standard::Rook)
    }
}
