use crate::direction::{
    ray::{RayBuilder, RaySet},
    Cardinal,
};

use super::PieceKind;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rook;

impl PieceKind for Rook {
    const VALUE: usize = 5;

    fn add_rays<'rays>(&self, set: &'rays mut RaySet) -> &'rays mut RaySet {
        set.add_many(Cardinal::ARRAY.map(RayBuilder::new))
    }
}
