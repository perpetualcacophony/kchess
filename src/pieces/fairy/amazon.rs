use crate::pieces::{standard, PrimitivePiece};

pub struct Amazon;

impl PrimitivePiece for Amazon {
    const VALUE: usize = 11;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(standard::Queen).add_piece(standard::Knight)
    }
}
