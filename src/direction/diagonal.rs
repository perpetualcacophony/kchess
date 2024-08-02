use super::{Direction, Step};

pub struct Diagonal {
    north: bool,
    east: bool,
}

impl Diagonal {
    pub const NORTHEAST: Self = Self::new(true, true);
    pub const SOUTHEAST: Self = Self::new(false, true);
    pub const SOUTHWEST: Self = Self::new(false, false);
    pub const NORTHWEST: Self = Self::new(true, false);

    pub const ARRAY: [Diagonal; 4] = [
        Self::NORTHEAST,
        Self::SOUTHEAST,
        Self::SOUTHWEST,
        Self::NORTHWEST,
    ];
}

impl Diagonal {
    pub const fn new(north: bool, east: bool) -> Self {
        Self { north, east }
    }
}

impl Direction for Diagonal {
    fn as_step(&self) -> Step {
        Step::new(
            if self.north { 1 } else { -1 },
            if self.east { 1 } else { -1 },
        )
    }
}

impl std::ops::Not for Diagonal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.north, !self.east)
    }
}
