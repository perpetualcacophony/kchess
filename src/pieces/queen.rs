use crate::direction::{self, cardinal, diagonal, ray::RayOwned, Cardinal, Direction};

pub type QueenDirection = Direction<direction::OneOrTwo<Cardinal>>;

pub fn directions() -> [QueenDirection; 8] {
    [
        cardinal::ARRAY.map(|dir| dir.map(direction::OneOrTwo::One)),
        diagonal::ARRAY.map(|dir| dir.map(direction::OneOrTwo::Two)),
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub type QueenRay = RayOwned;

pub fn rays() -> [QueenRay; 8] {
    directions().map(|direction| QueenRay::new(None, direction.into_boxed()))
}
