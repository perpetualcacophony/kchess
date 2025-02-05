use crate::direction::{
    ray::{self, set::Builder as RaySetBuilder},
    Cardinal,
};

use super::PrimitivePiece;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rook;

impl PrimitivePiece for Rook {
    const VALUE: usize = 5;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_many(Cardinal::ARRAY.map(ray::Builder::new))
    }
}
