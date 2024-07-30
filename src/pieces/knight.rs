use crate::direction::{ray::Ray, Cardinal, DirectionArray};

pub fn from_long(long: Cardinal) -> [DirectionArray<3>; 2] {
    long.perpendicular().map(|cardinal| new(long, cardinal))
}

pub fn new(long: Cardinal, short: Cardinal) -> DirectionArray<3> {
    assert!(long.perpendicular_to(short));

    DirectionArray::new([long, long, short])
}

pub fn directions() -> [DirectionArray<3>; 8] {
    Cardinal::ARRAY.map(from_long).concat().try_into().unwrap()
}

pub type KnightRay = Ray<DirectionArray<3>>;

pub fn rays() -> [KnightRay; 8] {
    directions().map(KnightRay::once)
}
