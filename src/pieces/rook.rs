use crate::direction::{
    cardinal,
    ray::{RayBuilder, RaySet, RayStatic},
};

pub type RookRay = RayStatic;

pub fn rays() -> [RookRay; 4] {
    cardinal::ARRAY.map(|direction| RookRay::new(None, direction.as_slice()))
}

pub fn rays_new() -> RaySet {
    RaySet::new().with_many(cardinal::ARRAY.map(|cardinal| RayBuilder::new(cardinal.boxed())))
}
