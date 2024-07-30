use super::{Cardinal, Direction, DirectionArray, DirectionStruct};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Diagonal {
    north: bool,
    east: bool,
}

impl Diagonal {
    pub const NORTHEAST: Self = Self::new(true, true);
    pub const SOUTHEAST: Self = Self::new(false, true);
    pub const SOUTHWEST: Self = Self::new(false, false);
    pub const NORTHWEST: Self = Self::new(true, false);

    pub const ARRAY: [Self; 4] = [
        Self::NORTHEAST,
        Self::SOUTHEAST,
        Self::SOUTHWEST,
        Self::NORTHWEST,
    ];

    const fn new(north: bool, east: bool) -> Self {
        Self { north, east }
    }

    fn as_cardinals(self) -> [Cardinal; 2] {
        let north_south = if self.north {
            Cardinal::North
        } else {
            Cardinal::South
        };

        let east_west = if self.east {
            Cardinal::East
        } else {
            Cardinal::West
        };

        [north_south, east_west]
    }

    const fn opposite(&self) -> Self {
        Self::new(!self.north, !self.east)
    }
}

impl std::ops::Not for Diagonal {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}

impl Direction for Diagonal {
    fn opposite(self) -> Self {
        !self
    }

    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        self.as_cardinals()
            .into_iter()
            .fold(start, |space, cardinal| cardinal.next_space(space))
    }

    fn perpendicular(self) -> [Self; 2] {
        [
            Self::new(self.north, !self.east),
            Self::new(!self.north, self.east),
        ]
    }
}

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
