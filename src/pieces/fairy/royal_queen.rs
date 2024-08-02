use crate::pieces::PrimitivePiece;

pub struct RoyalQueen;

impl PrimitivePiece for RoyalQueen {
    const VALUE: usize = <crate::pieces::King as PrimitivePiece>::VALUE;
    const VALID_PROMOTION: bool = false;
    const CHECKMATE_POSSIBLE: bool = true;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(crate::pieces::Queen)
    }
}
