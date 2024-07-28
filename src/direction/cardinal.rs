use super::Direction;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub const NORTH: Self = Self::North;
    pub const EAST: Self = Self::East;
    pub const SOUTH: Self = Self::South;
    pub const WEST: Self = Self::West;

    pub const ARRAY: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];
}

impl Direction for Cardinal {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        start.cardinal(self)
    }

    fn perpendicular(self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }
}

impl std::ops::Not for Cardinal {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}
