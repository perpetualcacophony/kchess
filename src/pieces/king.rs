use crate::direction::ray::RaySetBuilder;

use super::{PrimitivePiece, Queen};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct King;

impl PrimitivePiece for King {
    const VALUE: usize = usize::MAX;
    const VALID_PROMOTION: bool = false;
    const CHECKMATE_POSSIBLE: bool = true;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(Queen).map(|ray| ray.limit(1))
    }
}
