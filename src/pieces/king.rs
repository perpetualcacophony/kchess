use crate::direction::{ray::RayOwned, Cardinal, OneOrTwo};

use super::queen::{self, QueenDirection};

pub type KingRay = RayOwned;

pub fn rays() -> [KingRay; 8] {
    queen::directions().map(|direction| KingRay::new(Some(1), direction.into_owned()))
}
