use super::{Cardinal, DirectionDiagonal};

pub fn try_new(a: Cardinal, b: Cardinal) -> Option<DirectionDiagonal> {
    a.perpendicular_to(b).then(|| DirectionDiagonal::new(a, b))
}

pub const fn new(a: Cardinal, b: Cardinal) -> DirectionDiagonal {
    assert!(!a.equal_to(b));

    DirectionDiagonal::new(a, b)
}

pub const fn new_ne(north: bool, east: bool) -> DirectionDiagonal {
    let north_south = if north {
        Cardinal::NORTH
    } else {
        Cardinal::SOUTH
    };

    let east_west = if east { Cardinal::EAST } else { Cardinal::WEST };

    new(north_south, east_west)
}

pub const NORTHEAST: &DirectionDiagonal = &new_ne(true, true);
pub const SOUTHEAST: &DirectionDiagonal = &new_ne(false, true);
pub const SOUTHWEST: &DirectionDiagonal = &new_ne(false, false);
pub const NORTHWEST: &DirectionDiagonal = &new_ne(true, false);

pub const ARRAY: [&DirectionDiagonal; 4] = [NORTHEAST, SOUTHEAST, SOUTHWEST, NORTHWEST];
