use crate::direction::{
    ray::{RayBuilder, RaySet},
    Cardinal,
};

pub fn rays() -> RaySet {
    RaySet::new().with_many(Cardinal::ARRAY.map(RayBuilder::new))
}
