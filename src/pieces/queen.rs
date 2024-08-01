use crate::direction::{self, cardinal, diagonal, ray::RaySet, Cardinal, Direction};

use super::{bishop, rook};

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

pub fn rays() -> RaySet {
    RaySet::new()
        .with_set(bishop::rays())
        .with_set(rook::rays())
}
