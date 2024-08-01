use crate::direction::{
    cardinal,
    ray::{RayBuilder, RaySet, RayStatic},
};

pub type RookRay = RayStatic;

pub fn rays() -> RaySet {
    RaySet::new().with_many(cardinal::ARRAY.map(|cardinal| RayBuilder::new(cardinal.boxed())))
}
