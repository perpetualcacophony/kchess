use crate::direction::ray::set::Builder as RaySetBuilder;

use super::{PrimitivePiece, Queen};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct King;

impl PrimitivePiece for King {
    const VALUE: usize = usize::MAX;
    const VALID_PROMOTION: bool = false;
    const CHECKMATE_POSSIBLE: bool = true;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_piece(Queen).map(|ray| ray.some_limit(1))
    }
}
