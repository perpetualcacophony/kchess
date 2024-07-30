use crate::direction::{diagonal, ray::RayStatic, DirectionArray};

pub type BishopRay = RayStatic<DirectionArray<2>>;

pub fn rays() -> [BishopRay; 4] {
    diagonal::ARRAY.map(BishopRay::no_limit)
}
