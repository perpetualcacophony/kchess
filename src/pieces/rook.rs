use crate::direction::{
    cardinal,
    ray::{RayBuilder, RayStatic, Rays},
};

pub type RookRay = RayStatic;

pub fn rays() -> [RookRay; 4] {
    cardinal::ARRAY.map(|direction| RookRay::new(None, direction.as_slice()))
}

pub fn rays_new() -> Rays {
    Rays::new().with_many(cardinal::ARRAY.map(|cardinal| RayBuilder::new(cardinal.boxed())))
}
