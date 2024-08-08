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
    cardinal: Cardinal,
    right: bool,
}

impl KnightDirection {
    pub const fn new(cardinal: Cardinal, right: bool) -> Self {
        Self { cardinal, right }
    }

    pub const fn from_long(long: Cardinal) -> [Self; 2] {
        [Self::new(long, true), Self::new(long, false)]
    }

    pub fn array() -> [Self; 8] {
        Cardinal::ARRAY
            .map(Self::from_long)
            .concat()
            .try_into()
            .unwrap()
    }

    fn short_cardinal(&self) -> Cardinal {
        self.cardinal.rotate(self.right, 1)
    }
}

impl Direction for KnightDirection {
    fn as_step(&self) -> crate::direction::Step {
        self.cardinal.as_step() * 2 + self.short_cardinal().as_step()
    }

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self.cardinal == cardinal || self.short_cardinal() == cardinal
    }

    fn parse_step(step: crate::direction::Step) -> Option<Self>
    where
        Self: Sized,
    {
        if step.ranks.checked_abs() == Some(2) || step.files.checked_abs() == Some(1) {
            let cardinal = if step.ranks.is_positive() {
                Cardinal::NORTH
            } else {
                Cardinal::SOUTH
            };

            let right = !step.files.is_positive();

            Some(Self::new(cardinal, right))
        } else if step.files.checked_abs() == Some(2) || step.ranks.checked_abs() == Some(1) {
            let cardinal = if step.files.is_positive() {
                Cardinal::EAST
            } else {
                Cardinal::WEST
            };

            let right = !step.ranks.is_positive();

            Some(Self::new(cardinal, right))
        } else {
            None
        }
    }

    fn as_cardinals(&self) -> impl IntoIterator<Item = Cardinal> {
        [self.cardinal, self.cardinal, self.short_cardinal()]
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Knight;

impl PrimitivePiece for Knight {
    const VALUE: usize = 2;

    fn add_rays(set: &mut RaySetBuilder) -> &mut RaySetBuilder {
        set.add_many(
            KnightDirection::array().map(|direction| ray::Builder::new(direction).some_limit(1)),
        )
    }
}
