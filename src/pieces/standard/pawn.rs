use crate::direction::{
    ray::{RayBuilder, RaySetBuilder},
    Cardinal, Diagonal,
};

use super::PrimitivePiece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pawn;

impl PrimitivePiece for Pawn {
    const VALUE: usize = 1;
    const CAN_PROMOTE: bool = true;
    const VALID_PROMOTION: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_many(Diagonal::ARRAY.map(|direction| RayBuilder::new(direction).once()))
            .add_many(Cardinal::ARRAY.map(|direction| RayBuilder::new(direction).once()))
            .add_many(Cardinal::ARRAY.map(|direction| RayBuilder::new(direction).some_limit(2)))
    }

    fn ray_enabled(piece: &crate::components::Piece<'_>, ray: &crate::direction::Ray) -> bool {
        if !ray.step().contains_cardinal(piece.side.forward_cardinal()) {
            return false;
        }

        if ray.step().try_direction::<Cardinal>().is_some()
            && ((*piece.moved && ray.limit() == Some(2))
                || (!piece.moved && ray.limit() == Some(1)))
        {
            return false;
        }

        true
    }
}
