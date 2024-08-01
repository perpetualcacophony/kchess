use crate::direction::{cardinal, ray::RayStatic};

pub type RookRay = RayStatic;

pub fn rays() -> [RookRay; 4] {
    cardinal::ARRAY.map(|direction| RookRay::new(None, direction.as_slice()))
}
