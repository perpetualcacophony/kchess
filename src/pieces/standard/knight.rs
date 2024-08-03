use crate::{
    direction::{
        ray::{self, set::Builder as RaySetBuilder},
        Cardinal,
    },
    Direction,
};

use super::PrimitivePiece;

#[derive(Clone, Copy, Debug)]
pub struct KnightDirection {
    long: Cardinal,
    short: Cardinal,
}

impl KnightDirection {
    pub fn try_new(long: Cardinal, short: Cardinal) -> Option<Self> {
        long.perpendicular_to(short)
            .then_some(Self::new(long, short))
    }

    pub fn new(long: Cardinal, short: Cardinal) -> Self {
        assert!(long.perpendicular_to(short));
        Self { long, short }
    }

    pub fn from_long(long: Cardinal) -> [Self; 2] {
        long.perpendicular().map(|short| Self::new(long, short))
    }

    pub fn array() -> [Self; 8] {
        Cardinal::ARRAY
            .map(Self::from_long)
            .concat()
            .try_into()
            .unwrap()
    }
}

impl Direction for KnightDirection {
    fn as_step(&self) -> crate::direction::Step {
        self.long.as_step() * 2 + self.short.as_step()
    }

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self.long == cardinal || self.short == cardinal
    }

    fn parse_step(step: crate::direction::Step) -> Option<Self>
    where
        Self: Sized,
    {
        if step.ranks.checked_abs() == Some(2) || step.files.checked_abs() == Some(1) {
            let long = if step.ranks.is_positive() {
                Cardinal::NORTH
            } else {
                Cardinal::SOUTH
            };

            let short = if step.files.is_positive() {
                Cardinal::EAST
            } else {
                Cardinal::WEST
            };

            Some(Self::new(long, short))
        } else if step.files.checked_abs() == Some(2) || step.ranks.checked_abs() == Some(1) {
            let long = if step.files.is_positive() {
                Cardinal::NORTH
            } else {
                Cardinal::SOUTH
            };

            let short = if step.ranks.is_positive() {
                Cardinal::EAST
            } else {
                Cardinal::WEST
            };

            Some(Self::new(long, short))
        } else {
            None
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Knight;

impl PrimitivePiece for Knight {
    const VALUE: usize = 2;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        set.add_many(
            KnightDirection::array().map(|direction| ray::Builder::new(direction).some_limit(1)),
        )
    }
}
