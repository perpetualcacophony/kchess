use crate::{
    direction::ray::{Ray, RayBuilder as Builder},
    pieces::PrimitivePiece,
};

use super::RaySet;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RaySetBuilder {
    builders: Vec<Builder>,
}

impl RaySetBuilder {
    pub fn new(inner: impl FnOnce(&mut Self)) -> Self {
        let mut new = Self::default();
        inner(&mut new);
        new
    }

    pub fn map<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(Builder) -> Builder,
    {
        self.builders
            .iter_mut()
            .for_each(|builder| *builder = f(*builder));
        self
    }

    pub fn build(self) -> RaySet {
        RaySet {
            rays: self.builders.into_iter().map(Ray::from_builder).collect(),
        }
    }

    pub fn add(&mut self, builder: Builder) -> &mut Self {
        if let Some(index) = self
            .builders
            .iter()
            .position(|ray| ray.step == builder.step)
        {
            self.builders.remove(index);
        }

        self.builders.push(builder);

        self
    }

    pub fn add_many(&mut self, builders: impl IntoIterator<Item = Builder>) -> &mut Self {
        builders.into_iter().for_each(|builder| {
            self.add(builder);
        });
        self
    }

    pub fn add_piece<P: PrimitivePiece>(&mut self, kind: impl std::borrow::Borrow<P>) -> &mut Self {
        P::add_rays(kind.borrow(), self)
    }
}
