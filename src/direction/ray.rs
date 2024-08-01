use super::{DirectionExt, DirectionSlice};
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

pub mod set;
pub use set::RaySet;

pub type RayStatic = Ray<'static>;

pub struct Ray<'a> {
    limit: Option<usize>,
    direction: DirectionSlice<'a>,
}

impl<'a> Ray<'a> {
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

    pub fn cast(self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + 'a {
        self.into_iter().scan(start, move |start, dir| {
            *start = dir.next_space(*start);

            Some(*start)
        })
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
