use crate::pieces::PrimitivePiece;

pub struct Knook;

impl PrimitivePiece for Knook {
    const VALUE: usize = 8;

    fn add_rays<'rays>(
        &self,
        set: &'rays mut crate::direction::ray::RaySetBuilder,
    ) -> &'rays mut crate::direction::ray::RaySetBuilder {
        set.add_piece(crate::pieces::Knight)
            .add_piece(crate::pieces::Rook)
    }
}
