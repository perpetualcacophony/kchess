use crate::{
    direction::{Diagonal, Ray},
    UncheckedSpace,
};

pub type BishopRay = Ray<Diagonal>;

pub fn rays(start: UncheckedSpace) -> [BishopRay; 4] {
    BishopRay::map_array(start, Diagonal::ARRAY)
}
