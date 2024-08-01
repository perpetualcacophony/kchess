use crate::direction::{
    ray::{RayBuilder, RaySet},
    Diagonal,
};

pub fn rays() -> RaySet {
    RaySet::new().with_many(Diagonal::ARRAY.map(RayBuilder::new))
}
