use crate::UncheckedSpace;

use super::{Cast, Ray};

mod builder;
pub use builder::RaySetBuilder;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RaySet {
    #[cfg(not(feature = "smallvec"))]
    rays: Vec<Ray>,

    #[cfg(feature = "smallvec")]
    rays: smallvec::SmallVec<[Ray; 8]>,
}

impl RaySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn from_builder(inner: impl FnOnce(&mut RaySetBuilder)) -> Self {
        RaySetBuilder::new(inner).build()
    }

    pub fn cast(&self, start: UncheckedSpace) -> impl Iterator<Item = (&Ray, Cast<'_>)> + '_ {
        self.iter().map(move |ray| (ray, ray.cast(start)))
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
    inner: std::slice::Iter<'a, Ray>,
}

impl<'a> Iter<'a> {
    fn new(set: &'a RaySet) -> Self {
        Self {
            inner: set.rays.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Ray;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
