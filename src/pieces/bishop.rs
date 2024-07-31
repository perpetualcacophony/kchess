use crate::direction::{diagonal, ray::RayStatic, DirectionDiagonal};

pub type BishopRay = RayStatic<DirectionDiagonal>;

pub fn rays() -> [BishopRay; 4] {
    diagonal::ARRAY.map(|dir| BishopRay::new(None, dir))
}
