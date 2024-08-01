use crate::direction::{
    ray::{RayBuilder, RayOwned, RaySet},
    Cardinal, DirectionArray,
};

pub fn from_long(long: Cardinal) -> [DirectionArray<3>; 2] {
    long.perpendicular().map(|cardinal| new(long, cardinal))
}

pub fn new(long: Cardinal, short: Cardinal) -> DirectionArray<3> {
    assert!(long.perpendicular_to(short));

    DirectionArray::from_cardinals([long, long, short])
}

pub fn directions() -> [DirectionArray<3>; 8] {
    Cardinal::ARRAY.map(from_long).concat().try_into().unwrap()
}

pub type KnightRay = RayOwned;

pub fn rays() -> [KnightRay; 8] {
    directions().map(|direction| RayOwned::new(Some(1), direction.as_slice().into_boxed()))
}

pub fn rays_new() -> RaySet {
    RaySet::new().with_many(directions().map(|direction| RayBuilder::new(direction.boxed()).limit(1)))
}
