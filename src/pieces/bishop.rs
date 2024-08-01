use crate::direction::{
    diagonal,
    ray::{RayBuilder, RaySlice, RayStatic, Rays},
    DirectionArray, DirectionDiagonal,
};

pub type BishopRay = RayStatic;

pub fn rays() -> [BishopRay; 4] {
    diagonal::ARRAY.map(|dir| BishopRay::new(None, dir.as_slice()))
}

pub fn rays_new() -> Rays {
    let mut rays = Rays::new();

    rays.insert_many(
        diagonal::ARRAY
            .map(|dir| dir.as_slice().into_boxed())
            .map(RayBuilder::new),
    );

    rays
}
