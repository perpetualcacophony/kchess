use crate::direction::{cardinal, ray::RayStatic, DirectionArray};

pub type RookRay = RayStatic<DirectionArray<1>>;

pub fn rays() -> [RookRay; 4] {
    cardinal::ARRAY.map(RookRay::no_limit)
}
