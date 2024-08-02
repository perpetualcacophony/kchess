use crate::direction::{
    ray::{RayBuilder, RaySetBuilder},
    Diagonal,
};

use super::PrimitivePiece;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bishop;

impl PrimitivePiece for Bishop {
    const VALUE: usize = 3;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_many(Diagonal::ARRAY.map(RayBuilder::new))
    }
}
