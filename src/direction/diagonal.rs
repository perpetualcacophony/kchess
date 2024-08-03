use super::{Cardinal, Direction, Step};

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

    pub const fn contains(&self, cardinal: Cardinal) -> bool {
        match cardinal {
            Cardinal::North => self.north,
            Cardinal::East => self.east,
            Cardinal::South => !self.north,
            Cardinal::West => !self.east,
        }
    }
}

impl Direction for Diagonal {
    fn as_step(&self) -> Step {
        Step::new(
            if self.north { 1 } else { -1 },
            if self.east { 1 } else { -1 },
        )
    }

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self.contains(cardinal)
    }

    fn parse_step(step: Step) -> Option<Self>
    where
        Self: Sized,
    {
        if step.ranks == 0 || step.files == 0 {
            None
        } else {
            Some({
                let north = step.ranks.checked_abs() == Some(1);
                let east = step.files.checked_abs() == Some(1);
                Self::new(north, east)
            })
        }
    }
}

impl std::ops::Not for Diagonal {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(!self.north, !self.east)
    }
}
