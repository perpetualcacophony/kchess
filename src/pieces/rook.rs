use crate::direction::{
    ray::{RayBuilder, RaySetBuilder},
    Cardinal,
};

use super::PrimitivePiece;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rook;

impl PrimitivePiece for Rook {
    const VALUE: usize = 5;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_many(Cardinal::ARRAY.map(RayBuilder::new))
    }
}
