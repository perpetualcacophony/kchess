use crate::direction::ray::Ray;

use super::queen::{self, QueenDirection};

pub type KingRay = Ray<QueenDirection>;

pub fn rays() -> [KingRay; 8] {
    queen::directions().map(KingRay::once)
}
