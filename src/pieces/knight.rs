use crate::{
    direction::{
        ray::{RayBuilder, RaySet},
        Cardinal,
    },
    Direction,
};

use super::PieceKind;

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
}

pub struct Knight;

impl PieceKind for Knight {
    const VALUE: usize = 2;

    fn add_rays<'rays>(&self, set: &'rays mut RaySet) -> &'rays mut RaySet {
        set.add_many(KnightDirection::array().map(|direction| RayBuilder::new(direction).limit(1)))
    }
}
