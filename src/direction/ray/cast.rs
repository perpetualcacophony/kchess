use crate::Space;

use super::{Ray, Steps};

pub struct Cast<'ray, 'start> {
    partial: PartialCast<'ray>,
    pub meta: CastMeta<'ray, 'start>,
}

impl<'ray, 'start> Cast<'ray, 'start> {
    pub(super) fn new(ray: &'ray Ray, start: &'start Space) -> Self {
        Self {
            partial: PartialCast::new(ray, *start),
            meta: CastMeta { ray, start },
        }
    }

    pub fn meta(&self) -> &CastMeta<'ray, 'start> {
        &self.meta
    }
}

impl<'ray> Iterator for Cast<'ray, '_> {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        self.partial.next()
    }
}

impl std::iter::FusedIterator for Cast<'_, '_> {}

impl<'ray, 'start> std::ops::Deref for Cast<'ray, 'start> {
    type Target = CastMeta<'ray, 'start>;

    fn deref(&self) -> &Self::Target {
        self.meta()
    }
}

pub struct CastMeta<'ray, 'start> {
    pub ray: &'ray Ray,
    pub start: &'start Space,
}

impl CastMeta<'_, '_> {
    pub fn intersects(&self, space: &Space) -> bool {
        self.start.distance_step(space).is_some_and(|step| {
            step.div_exact(self.ray.step()).is_some_and(|quotient| {
                if let Some(limit) = self.ray.limit() {
                    quotient.unsigned_abs() <= limit
                } else {
                    true
                }
            })
        })
    }

    pub fn ray(&self) -> &Ray {
        self.ray
    }

    pub fn start(&self) -> &Space {
        self.start
    }
}

pub struct PartialCast<'ray> {
    steps: Steps<'ray>,
    current: Space,
}

impl<'ray> PartialCast<'ray> {
    fn new(ray: &'ray Ray, start: Space) -> Self {
        Self {
            steps: ray.steps(),
            current: start,
        }
    }
}

impl<'ray> Iterator for PartialCast<'ray> {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.steps.next()?.next_space(&self.current)?;
        Some(self.current)
    }
}

impl std::iter::FusedIterator for PartialCast<'_> {}
