use crate::game::components::Piece;

use super::{Cast, Ray};

mod builder;
pub use builder::RaySetBuilder as Builder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RaySet {
    #[cfg(not(feature = "smallvec"))]
    rays: Vec<Ray>,

    #[cfg(feature = "smallvec")]
    rays: smallvec::SmallVec<[Ray; 8]>,

    filter: fn(&Piece, &Ray) -> bool,
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

    pub fn cast<'a, 'b: 'a>(
        &'a self,
        piece: &'b Piece,
    ) -> impl Iterator<Item = (&'a Ray, Cast<'a>)> + 'a {
        self.iter()
            .filter(move |ray| (self.filter)(piece, ray))
            .map(move |ray| (ray, ray.cast(&piece.space)))
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
