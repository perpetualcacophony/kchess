use super::{Cardinal, Direction, Step};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
        if step.ranks.checked_abs() != Some(1) || step.files.checked_abs() != Some(1) {
            None
        } else {
            Some({
                let north = step.ranks.is_positive();
                let east = step.files.is_positive();
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

#[cfg(test)]
mod tests {
    #[macro_use]
    mod macros;

    use crate::Direction;

    use super::Diagonal;

    contains_cardinal! {
        NORTHEAST {
            NORTH true
            EAST true
            SOUTH false
            WEST false
        }

        NORTHWEST {
            NORTH true
            EAST false
            SOUTH false
            WEST true
        }

        SOUTHWEST {
            NORTH false
            EAST false
            SOUTH true
            WEST true
        }

        SOUTHEAST {
            NORTH false
            EAST true
            SOUTH true
            WEST false
        }
    }

    opposite! {
        NORTHEAST SOUTHWEST
        NORTHWEST SOUTHEAST
        SOUTHWEST NORTHEAST
        SOUTHEAST NORTHWEST
    }

    mod parse_step {
        use super::Diagonal;
        use crate::{direction::Step, Direction};

        #[test]
        fn parse_1_1() {
            assert_eq!(
                Diagonal::parse_step(Step::new(1, 1)),
                Some(Diagonal::NORTHEAST)
            )
        }

        #[test]
        fn parse_1_neg1() {
            assert_eq!(
                Diagonal::parse_step(Step::new(1, -1)),
                Some(Diagonal::NORTHWEST)
            )
        }

        #[test]
        fn parse_neg1_1() {
            assert_eq!(
                Diagonal::parse_step(Step::new(-1, 1)),
                Some(Diagonal::SOUTHEAST)
            )
        }

        #[test]
        fn parse_neg1_neg1() {
            assert_eq!(
                Diagonal::parse_step(Step::new(-1, -1)),
                Some(Diagonal::SOUTHWEST)
            )
        }

        #[test]
        #[should_panic]
        fn parse_0_0() {
            assert!(Diagonal::parse_step(Step::new(0, 0)).is_some())
        }

        #[test]
        #[should_panic]
        fn parse_2_3() {
            assert!(Diagonal::parse_step(Step::new(2, 3)).is_some())
        }
    }

    #[test]
    fn to_step_then_parse() {
        assert_eq!(
            Diagonal::NORTHEAST.as_step().try_direction::<Diagonal>(),
            Some(Diagonal::NORTHEAST)
        )
    }
}
