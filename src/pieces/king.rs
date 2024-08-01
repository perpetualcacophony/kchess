use crate::direction::ray::{Ray, RaySet};

use super::{PieceKind, Queen};

pub struct King;

impl PieceKind for King {
    const VALUE: usize = usize::MAX;
    const VALID_PROMOTION: bool = false;
    const CHECKMATE_POSSIBLE: bool = true;

    fn add_rays<'rays>(&self, set: &'rays mut RaySet) -> &'rays mut RaySet {
        set.add_piece(Queen)
            .map(|ray| *ray = Ray::from_builder(ray.into_builder().limit(1)))
    }
}
