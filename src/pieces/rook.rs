use crate::direction::{Cardinal, InfiniteRay};

pub type RookRay = InfiniteRay<Cardinal>;

pub fn rays() -> [RookRay; 4] {
    RookRay::map_array(Cardinal::ARRAY)
}
