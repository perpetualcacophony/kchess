use crate::direction::ray::{RayOwned, Rays};

use super::queen;

pub type KingRay = RayOwned;

pub fn rays() -> [KingRay; 8] {
    queen::directions().map(|direction| KingRay::new(Some(1), direction.into_boxed()))
}

pub fn rays_new() -> Rays {
    queen::rays_new().map(|_| Some(1))
}
