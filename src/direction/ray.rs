use super::{DirectionBoxed, DirectionExt, DirectionSlice};
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RaySet {
    map: std::collections::HashMap<DirectionBoxed, Option<usize>>,
}

impl RaySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> RaySetIter {
        RaySetIter::new(self)
    }

    pub fn map<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(Option<usize>) -> Option<usize>,
    {
        self.map.values_mut().for_each(|limit| *limit = f(*limit));
        self
    }

    pub fn cast(
        &self,
        start: UncheckedSpace,
    ) -> impl Iterator<Item = impl Iterator<Item = UncheckedSpace> + '_> + '_ {
        self.iter().map(move |ray| ray.cast(start))
    }

    pub fn add(&mut self, builder: RayBuilder<DirectionBoxed>) {
        self.map.insert(builder.direction, builder.limit);
    }

    pub fn add_many(&mut self, builders: impl IntoIterator<Item = RayBuilder<DirectionBoxed>>) {
        builders.into_iter().for_each(|builder| self.add(builder))
    }

    pub fn add_set(&mut self, other: RaySet) {
        other.map.into_iter().for_each(|(direction, limit)| {
            self.map.insert(direction, limit);
        })
    }

    pub fn with(mut self, builder: RayBuilder<DirectionBoxed>) -> Self {
        self.add(builder);
        self
    }

    pub fn with_many(
        mut self,
        builders: impl IntoIterator<Item = RayBuilder<DirectionBoxed>>,
    ) -> Self {
        self.add_many(builders);
        self
    }

    pub fn with_set(mut self, other: RaySet) -> Self {
        self.add_set(other);
        self
    }
}

impl<'a> IntoIterator for &'a RaySet {
    type IntoIter = RaySetIter<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct RaySetIter<'a> {
    inner: std::collections::hash_map::Iter<'a, DirectionBoxed, Option<usize>>,
}

impl<'a> RaySetIter<'a> {
    fn new(set: &'a RaySet) -> Self {
        Self {
            inner: set.map.iter(),
        }
    }
}

impl<'a> Iterator for RaySetIter<'a> {
    type Item = RaySlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(direction, limit)| RaySlice::new(*limit, direction.as_slice()))
    }
}
