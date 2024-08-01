use crate::direction::ray::RayOwned;

use super::queen;

pub type KingRay = RayOwned;

pub fn rays() -> [KingRay; 8] {
    queen::directions().map(|direction| KingRay::new(Some(1), direction.into_boxed()))
}
