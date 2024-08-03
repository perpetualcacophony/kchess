use super::{Direction, Step};

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

    pub fn rotate_cw(self, turns: usize) -> Self {
        if turns == 1 {
            self
        } else if turns == 2 {
            self.opposite()
        } else {
            let mut rotated = self;

            for _ in 0..turns {
                rotated = match rotated {
                    Self::NORTH => Self::EAST,
                    Self::EAST => Self::SOUTH,
                    Self::SOUTH => Self::WEST,
                    Self::WEST => Self::NORTH,
                }
            }

            rotated
        }
    }

    pub fn turns_cw(self, rhs: Self) -> usize {
        if self == rhs {
            0
        } else if self == rhs.opposite() {
            2
        } else if self.rotate_cw(1) == rhs {
            1
        } else if self.rotate_cw(3) == rhs {
            3
        } else {
            unreachable!()
        }
    }

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

impl Direction for Cardinal {
    fn as_step(&self) -> Step {
        let ranks = match *self {
            Self::NORTH => 1,
            Self::SOUTH => -1,
            _ => 0,
        };

        let files = match *self {
            Self::EAST => 1,
            Self::WEST => -1,
            _ => 0,
        };

        Step::new(ranks, files)
    }

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self == &cardinal
    }
}

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
