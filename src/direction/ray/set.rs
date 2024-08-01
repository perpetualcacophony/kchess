use crate::{direction::DirectionBoxed, UncheckedSpace};

use super::{RayBuilder, RaySlice};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RaySet {
    map: std::collections::HashMap<DirectionBoxed, Option<usize>>,
}

impl RaySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
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
    type IntoIter = Iter<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a> {
    inner: std::collections::hash_map::Iter<'a, DirectionBoxed, Option<usize>>,
}

impl<'a> Iter<'a> {
    fn new(set: &'a RaySet) -> Self {
        Self {
            inner: set.map.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = RaySlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(direction, limit)| RaySlice::new(*limit, direction.as_slice()))
    }
}
