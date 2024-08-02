use crate::pieces::{standard, PrimitivePiece};

pub struct Nightrider;

impl PrimitivePiece for Nightrider {
    const VALUE: usize = 5;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(standard::Knight)
            .map(|builder| builder.limit(None))
    }
}
