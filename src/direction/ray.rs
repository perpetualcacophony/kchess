use super::{DirectionBoxed, DirectionExt, DirectionSlice};
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

pub mod set;
pub use set::RaySet;

pub type RayStatic = RaySlice<'static>;

pub struct RaySlice<'a> {
    limit: Option<usize>,
    direction: DirectionSlice<'a>,
}

impl<'a> RaySlice<'a> {
    pub const fn new(limit: Option<usize>, direction: DirectionSlice<'a>) -> Self {
        Self { limit, direction }
    }

    pub const fn from_builder(builder: RayBuilder<DirectionSlice<'a>>) -> Self {
        Self::new(builder.limit, builder.direction)
    }

    pub const fn iter(&self) -> Iter<'a> {
        Iter::new(self.limit, self.direction)
    }

    pub const fn into_iter(self) -> Iter<'a> {
        Iter::new(self.limit, self.direction)
    }

    pub fn into_owned(self) -> RayOwned {
        RayOwned::new(self.limit, self.direction.into_boxed())
    }

    pub fn cast(self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + 'a {
        self.into_iter().scan(start, move |start, dir| {
            *start = dir.next_space(*start);

            Some(*start)
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RayOwned {
    limit: Option<usize>,
    direction: DirectionBoxed,
}

impl RayOwned {
    pub const fn new(limit: Option<usize>, direction: DirectionBoxed) -> Self {
        Self { limit, direction }
    }

    pub fn from_builder(builder: RayBuilder<DirectionBoxed>) -> Self {
        Self::new(builder.limit, builder.direction)
    }

    pub const fn as_borrowed(&self) -> RaySlice {
        RaySlice::new(self.limit, self.direction.as_slice())
    }

    pub const fn iter(&self) -> Iter {
        self.as_borrowed().into_iter()
    }

    pub fn cast(&self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + '_ {
        self.as_borrowed().cast(start)
    }
}

pub struct Iter<'ray> {
    limit: Option<usize>,
    direction: DirectionSlice<'ray>,
}

impl<'a> Iter<'a> {
    const fn new(limit: Option<usize>, direction: DirectionSlice<'a>) -> Self {
        Self { limit, direction }
    }
}

impl<'ray> Iterator for Iter<'ray> {
    type Item = DirectionSlice<'ray>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit.as_mut() {
            if *limit == 0 {
                return None;
            } else {
                *limit -= 1;
            }
        }

        Some(self.direction)
    }
}
