use crate::{
    direction::{Cardinal, Ray},
    UncheckedSpace,
};

pub type RookRay = Ray<Cardinal>;

pub fn rays(start: UncheckedSpace) -> [RookRay; 4] {
    RookRay::map_array(start, Cardinal::ARRAY)
}
