use crate::direction::{
    ray::{RayBuilder, RaySet},
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

pub fn rays() -> RaySet {
    RaySet::new()
        .with_many(directions().map(|direction| RayBuilder::new(direction.boxed()).limit(1)))
}
