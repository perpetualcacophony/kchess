use super::{Cardinal, DirectionArray};

pub fn try_new(a: Cardinal, b: Cardinal) -> Option<DirectionArray<2>> {
    a.perpendicular_to(b).then(|| DirectionArray::double(a, b))
}

pub const fn new(a: Cardinal, b: Cardinal) -> DirectionArray<2> {
    DirectionArray::double(a, b)
}

pub const fn new_ne(north: bool, east: bool) -> DirectionArray<2> {
    let north_south = if north {
        Cardinal::NORTH
    } else {
        Cardinal::SOUTH
    };

    let east_west = if east { Cardinal::EAST } else { Cardinal::WEST };

    new(north_south, east_west)
}

pub const NORTHEAST: &DirectionArray<2> = &new_ne(true, true);
pub const SOUTHEAST: &DirectionArray<2> = &new_ne(false, true);
pub const SOUTHWEST: &DirectionArray<2> = &new_ne(false, false);
pub const NORTHWEST: &DirectionArray<2> = &new_ne(true, false);

pub const ARRAY: [&DirectionArray<2>; 4] = [NORTHEAST, SOUTHEAST, SOUTHWEST, NORTHWEST];
