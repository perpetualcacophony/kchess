use crate::direction::ray::set::Builder as RaySetBuilder;
use crate::pieces::{standard, PrimitivePiece};

pub struct RoyalQueen;

impl PrimitivePiece for RoyalQueen {
    const VALUE: usize = <standard::King as PrimitivePiece>::VALUE;
    const VALID_PROMOTION: bool = false;
    const CHECKMATE_POSSIBLE: bool = true;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_piece::<standard::Queen>()
    }
}
