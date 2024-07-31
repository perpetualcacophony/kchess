use crate::direction::{cardinal, ray::RayStatic, DirectionCardinal};

pub type RookRay = RayStatic<DirectionCardinal>;

pub fn rays() -> [RookRay; 4] {
    cardinal::ARRAY.map(|direction| RookRay::new(None, direction))
}
