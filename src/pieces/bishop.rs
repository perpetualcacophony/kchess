use crate::direction::{
    diagonal,
    ray::{RayBuilder, RaySet, RayStatic},
};

pub type BishopRay = RayStatic;

pub fn rays() -> RaySet {
    RaySet::new().with_many(diagonal::ARRAY.map(|diagonal| RayBuilder::new(diagonal.boxed())))
}
