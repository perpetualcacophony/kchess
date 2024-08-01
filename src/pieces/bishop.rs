use crate::direction::{
    diagonal,
    ray::{RayBuilder, RayStatic, Rays},
};

pub type BishopRay = RayStatic;

pub fn rays() -> [BishopRay; 4] {
    diagonal::ARRAY.map(|dir| BishopRay::new(None, dir.as_slice()))
}

pub fn rays_new() -> Rays {
    Rays::new().with_many(diagonal::ARRAY.map(|diagonal| RayBuilder::new(diagonal.boxed())))
}
