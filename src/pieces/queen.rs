use crate::direction::{self, cardinal, diagonal, Cardinal, Direction, Ray};

pub type QueenDirection = Direction<direction::OneOrTwo<Cardinal>>;

pub fn directions() -> [QueenDirection; 8] {
    [
        cardinal::ARRAY.map(|dir| (*dir).map(direction::OneOrTwo::One)),
        diagonal::ARRAY.map(|dir| (*dir).map(direction::OneOrTwo::Two)),
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub type QueenRay = Ray<QueenDirection>;

pub fn rays() -> [QueenRay; 8] {
    directions().map(QueenRay::no_limit)
}
