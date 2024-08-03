use crate::direction::ray::set::Builder as RaySetBuilder;
use crate::pieces::{standard, PrimitivePiece};

pub struct Amazon;

impl PrimitivePiece for Amazon {
    const VALUE: usize = 12;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(standard::Queen).add_piece(standard::Knight)
    }
}
