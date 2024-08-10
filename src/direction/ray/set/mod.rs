use crate::{game::piece::PartialPiece, Space};

use super::{Cast, Ray};

mod builder;
pub use builder::RaySetBuilder as Builder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RaySet {
    #[cfg(not(feature = "smallvec"))]
    rays: Vec<Ray>,

    #[cfg(feature = "smallvec")]
    rays: smallvec::SmallVec<[Ray; 8]>,

    filter: fn(&PartialPiece, &Ray) -> bool,
}

impl Default for RaySet {
    fn default() -> Self {
        Self {
            rays: Default::default(),
            filter: |_, _| true,
        }
    }
}

impl RaySet {
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn new(inner: impl FnOnce(&mut Builder) -> &mut Builder) -> Self {
        Builder::new(inner).build()
    }

    pub fn enabled<'a, 'b: 'a>(&'a self, piece: &'b PartialPiece) -> impl Iterator<Item = &'a Ray> {
        self.iter().filter(move |ray| (self.filter)(piece, ray))
    }

    pub fn cast<'a, 'b: 'a>(
        &'a self,
        piece: &'b PartialPiece,
    ) -> impl Iterator<Item = Cast<'a, 'b>> + 'a {
        self.enabled(piece).map(move |ray| ray.cast(&piece.space))
    }

    pub fn intersection(&self, start: &Space, target: &Space) -> Option<&Ray> {
        self.iter().find(|ray| ray.intersects(start, target))
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
