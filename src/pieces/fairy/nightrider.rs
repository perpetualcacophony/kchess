use crate::direction::ray::set::Builder as RaySetBuilder;
use crate::pieces::{standard, PrimitivePiece};

pub struct Nightrider;

impl PrimitivePiece for Nightrider {
    const VALUE: usize = 5;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_piece::<standard::Knight>()
            .map(|builder| builder.limit(None))
    }
}
