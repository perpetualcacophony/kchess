use crate::direction::{Diagonal, InfiniteRay};

pub type BishopRay = InfiniteRay<Diagonal>;

pub fn rays() -> [BishopRay; 4] {
    BishopRay::map_array(Diagonal::ARRAY)
}
