use crate::{
    direction::ray::{Builder, Ray},
    pieces::PrimitivePiece,
};

use super::RaySet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaySetBuilder {
    builders: Vec<Builder>,
    filter: fn(&crate::components::Piece, &Ray) -> bool,
}

impl Default for RaySetBuilder {
    fn default() -> Self {
        Self {
            builders: Default::default(),
            filter: |_, _| true,
        }
    }
}

impl RaySetBuilder {
    pub fn new(inner: impl FnOnce(&mut Self) -> &mut Self) -> Self {
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
            filter: self.filter,
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
        P::add_rays(kind.borrow(), self).add_filter::<P>()
    }

    pub fn add_filter<P: PrimitivePiece>(&mut self) -> &mut Self {
        self.filter = P::ray_enabled;
        self
    }
}
