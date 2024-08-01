use super::{Cardinal, Direction, Step};

pub struct Diagonal {
    a: Cardinal,
    b: Cardinal,
}

impl Diagonal {
    pub const NORTHEAST: Self = Self::new_ne(true, true);
    pub const SOUTHEAST: Self = Self::new_ne(false, true);
    pub const SOUTHWEST: Self = Self::new_ne(false, false);
    pub const NORTHWEST: Self = Self::new_ne(true, false);

    pub const ARRAY: [Diagonal; 4] = [
        Self::NORTHEAST,
        Self::SOUTHEAST,
        Self::SOUTHWEST,
        Self::NORTHWEST,
    ];
}

impl Diagonal {
    pub fn try_new(a: Cardinal, b: Cardinal) -> Option<Self> {
        a.perpendicular_to(b).then(|| Diagonal::new(a, b))
    }

    pub const fn new(a: Cardinal, b: Cardinal) -> Self {
        assert!(a.perpendicular_to(b));
        Self { a, b }
    }

    pub const fn new_ne(north: bool, east: bool) -> Self {
        let north_south = if north {
            Cardinal::NORTH
        } else {
            Cardinal::SOUTH
        };

        let east_west = if east { Cardinal::EAST } else { Cardinal::WEST };

        Self::new(north_south, east_west)
    }
}

impl Direction for Diagonal {
    fn as_step(&self) -> Step {
        self.a.as_step() + self.b.as_step()
    }
}

impl std::ops::Not for Diagonal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.a, !self.b)
    }
}
