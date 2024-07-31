use super::{Cardinal, Direction};
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

pub type RayStatic<Collection> = RayBorrowed<'static, Collection>;

pub struct RayBorrowed<'a, Direction> {
    limit: Option<usize>,
    direction: &'a Direction,
}

impl<'a, Direction> RayBorrowed<'a, Direction> {
    pub const fn new(limit: Option<usize>, direction: &'a Direction) -> Self {
        Self { limit, direction }
    }

    pub const fn from_builder(builder: RayBuilder<&'a Direction>) -> Self {
        Self::new(builder.limit, builder.direction)
    }

    pub const fn iter(&self) -> Iter<Direction> {
        Iter::new(self.limit, self.direction)
    }

    pub const fn into_iter(self) -> Iter<'a, Direction> {
        Iter::new(self.limit, self.direction)
    }
}

impl<'a, Collection> RayBorrowed<'a, Direction<Collection>> {
    pub fn into_owned(self) -> RayOwned
    where
        Direction<Collection>: Clone,
        Collection: Into<Vec<Cardinal>>,
    {
        RayOwned::new(self.limit, self.direction.clone().into_owned())
    }

    pub fn cast(self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + 'a
    where
        for<'b> &'b Direction<Collection>: IntoIterator<Item = Cardinal>,
    {
        self.into_iter().scan(start, move |start, dir| {
            *start = dir.next_space(*start);

            Some(*start)
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RayOwned {
    limit: Option<usize>,
    direction: Direction,
}

impl RayOwned {
    pub const fn new(limit: Option<usize>, direction: Direction) -> Self {
        Self { limit, direction }
    }

    pub fn from_builder(builder: RayBuilder) -> Self {
        Self::new(builder.limit, builder.direction)
    }

    pub const fn as_borrowed(&self) -> RayBorrowed<Direction> {
        RayBorrowed::new(self.limit, &self.direction)
    }

    pub const fn iter(&self) -> Iter<Direction> {
        self.as_borrowed().into_iter()
    }

    pub fn cast(&self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + '_ {
        self.as_borrowed().cast(start)
    }
}

pub struct Iter<'ray, Direction> {
    limit: Option<usize>,
    direction: &'ray Direction,
}

impl<'a, Direction> Iter<'a, Direction> {
    const fn new(limit: Option<usize>, direction: &'a Direction) -> Self {
        Self { limit, direction }
    }
}

impl<'ray, Collection> Iterator for Iter<'ray, Direction<Collection>> {
    type Item = &'ray Direction<Collection>;

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

pub struct Rays {
    map: std::collections::HashMap<Direction, Option<usize>>,
}

impl Rays {
    pub fn insert(&mut self, builder: RayBuilder) {
        self.map.insert(builder.direction, builder.limit);
    }

    pub fn rays(&self) -> impl Iterator<Item = RayBorrowed<Direction>> {
        self.map
            .iter()
            .map(|(direction, limit)| RayBorrowed::new(*limit, direction))
    }

    pub fn cast(
        &self,
        start: UncheckedSpace,
    ) -> impl Iterator<Item = impl Iterator<Item = UncheckedSpace> + '_> {
        self.rays().map(move |ray| ray.cast(start))
    }
}
