use crate::direction::{
    ray::{self, set::Builder as RaySetBuilder},
    Diagonal,
};

use super::PrimitivePiece;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bishop;

impl PrimitivePiece for Bishop {
    const VALUE: usize = 3;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_many(Diagonal::ARRAY.map(ray::Builder::new))
    }
}
