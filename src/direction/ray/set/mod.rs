use std::borrow::Borrow;

use crate::{pieces::PieceKind, UncheckedSpace};

use super::{Cast, Ray, RayBuilder};

mod builder;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RaySet {
    rays: Vec<Ray>,
}

impl RaySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn map<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&mut Ray),
    {
        self.rays.iter_mut().for_each(f);
        self
    }

    pub fn cast(&self, start: UncheckedSpace) -> impl Iterator<Item = (&Ray, Cast<'_>)> + '_ {
        self.iter().map(move |ray| (ray, ray.cast(start)))
    }

    pub fn add(&mut self, builder: RayBuilder) -> &mut Self {
        if let Some(index) = self.rays.iter().position(|ray| ray.step == builder.step) {
            self.rays.remove(index);
        }

        self.rays.push(Ray::from_builder(builder));

        self
    }

    pub fn add_many(&mut self, builders: impl IntoIterator<Item = RayBuilder>) -> &mut Self {
        builders.into_iter().for_each(|builder| {
            self.add(builder);
        });
        self
    }

    pub fn add_set(&mut self, other: RaySet) -> &mut Self {
        other.into_iter().for_each(|ray| {
            self.add(ray.into_builder());
        });
        self
    }

    pub fn add_piece<P: PieceKind>(&mut self, kind: impl Borrow<P>) -> &mut Self {
        P::add_rays(kind.borrow(), self)
    }

    pub fn with(mut self, builder: RayBuilder) -> Self {
        self.add(builder);
        self
    }

    pub fn with_many(mut self, builders: impl IntoIterator<Item = RayBuilder>) -> Self {
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
