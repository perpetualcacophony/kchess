use crate::pieces::{standard, PrimitivePiece};

pub struct Knook;

impl PrimitivePiece for Knook {
    const VALUE: usize = 8;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(standard::Knight).add_piece(standard::Rook)
    }
}
