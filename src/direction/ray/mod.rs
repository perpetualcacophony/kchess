use std::iter::FusedIterator;

use super::step::Step;
use crate::Space;

mod builder;
pub use builder::RayBuilder as Builder;

pub mod set;
pub use set::RaySet as Set;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ray {
    limit: Option<usize>,
    step: Step,
    capture: bool,
}

impl Ray {
    pub const fn capture(&self) -> bool {
        self.capture
    }

    pub const fn from_builder(builder: Builder) -> Self {
        Self {
            limit: builder.limit,
            step: builder.step,
            capture: builder.capture,
        }
    }

    const fn steps(&self) -> Steps {
        Steps::new(self)
    }

    pub const fn step(&self) -> Step {
        self.step
    }

    pub const fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn cast<'start>(&self, start: &'start Space) -> Cast<'_, 'start> {
        Cast::new(self, start)
    }

    pub fn into_builder(self) -> Builder {
        Builder {
            limit: self.limit,
            capture: self.capture,
            step: self.step,
        }
    }

    pub fn intersects(&self, start: &Space, target: &Space) -> bool {
        start.distance_step(target).is_some_and(|step| {
            if step.ranks % self.step.ranks == 0 && step.files % self.step.files == 0 {
                if let Some(limit) = self.limit() {
                    step.ranks / self.step.ranks == limit as isize
                        && step.files / self.step.files == limit as isize
                } else {
                    true
                }
            } else {
                false
            }
        })
    }
}

struct Steps<'ray> {
    limit: Option<usize>,
    step: &'ray Step,
}

impl<'ray> Steps<'ray> {
    const fn new(ray: &'ray Ray) -> Self {
        Self {
            limit: ray.limit,
            step: &ray.step,
        }
    }
}

impl<'ray> Iterator for Steps<'ray> {
    type Item = &'ray Step;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit.as_mut() {
            if *limit == 0 {
                return None;
            } else {
                *limit -= 1;
            }
        }

        Some(self.step)
    }
}

pub struct Cast<'ray, 'start> {
    partial: PartialCast<'ray>,
    pub meta: CastMeta<'ray, 'start>,
}

impl<'ray, 'start> Cast<'ray, 'start> {
    fn new(ray: &'ray Ray, start: &'start Space) -> Self {
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

impl FusedIterator for Cast<'_, '_> {}

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

impl FusedIterator for PartialCast<'_> {}
