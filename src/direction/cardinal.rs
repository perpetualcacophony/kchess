use super::DirectionCardinal;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    pub const SLICE: &[Self] = &[Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    pub const fn perpendicular(self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }

    pub const fn perpendicular_to(self, rhs: Self) -> bool {
        matches!(
            (self, rhs),
            (Self::North | Self::South, Self::East | Self::West)
                | (Self::East | Self::West, Self::North | Self::South)
        )
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::SOUTH,
            Self::East => Self::WEST,
            Self::South => Self::NORTH,
            Self::West => Self::EAST,
        }
    }

    pub const fn equal_to(self, rhs: Self) -> bool {
        matches!(
            (self, rhs),
            (Self::North, Self::North)
                | (Self::East, Self::East)
                | (Self::South, Self::South)
                | (Self::West, Self::West)
        )
    }
}

impl std::ops::Not for Cardinal {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.opposite()
    }
}

pub const NORTH: &DirectionCardinal = &DirectionCardinal::new(Cardinal::NORTH);
pub const EAST: &DirectionCardinal = &DirectionCardinal::new(Cardinal::EAST);
pub const SOUTH: &DirectionCardinal = &DirectionCardinal::new(Cardinal::SOUTH);
pub const WEST: &DirectionCardinal = &DirectionCardinal::new(Cardinal::WEST);

pub const ARRAY: [&DirectionCardinal; 4] = [NORTH, EAST, SOUTH, WEST];

#[cfg(test)]
mod tests {
    #[macro_use]
    mod macros;

    use super::Cardinal;

    mod perpendicular {
        assert_perpendicular_to! {
            NORTH {
                NORTH false
                EAST true
                SOUTH false
                WEST true
            }

            EAST {
                NORTH true
                EAST false
                SOUTH true
                WEST false
            }

            SOUTH {
                NORTH false
                EAST true
                SOUTH false
                WEST true
            }

            WEST {
                NORTH true
                EAST false
                SOUTH true
                WEST false
            }
        }
    }
}
