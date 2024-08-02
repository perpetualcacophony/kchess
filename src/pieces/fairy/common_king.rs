use crate::pieces::PrimitivePiece;

pub struct CommonKing;

impl PrimitivePiece for CommonKing {
    const VALUE: usize = 4;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(crate::pieces::King)
    }
}
