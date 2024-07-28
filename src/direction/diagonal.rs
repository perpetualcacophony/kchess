use super::{Cardinal, Direction};

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
}

impl std::ops::Not for Diagonal {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}

impl Direction for Diagonal {
    fn opposite(self) -> Self {
        Self::new(!self.north, !self.east)
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
