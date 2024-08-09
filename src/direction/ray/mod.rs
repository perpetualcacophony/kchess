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

    pub fn cast(&self, start: &Space) -> Cast {
        Cast::new(self, start)
    }

    pub fn into_builder(self) -> Builder {
        Builder {
            limit: self.limit,
            capture: self.capture,
            step: self.step,
        }
    }

    pub fn in_path(&self, start: &Space, target: &Space) -> bool {
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

type CastInner<'ray> =
    std::iter::Scan<Steps<'ray>, Space, fn(&mut Space, <Steps as Iterator>::Item) -> Option<Space>>;

pub struct Cast<'ray> {
    inner: CastInner<'ray>,
}

impl<'ray> Cast<'ray> {
    fn new(ray: &'ray Ray, start: &Space) -> Self {
        Self {
            inner: ray.steps().scan(*start, |start, step| {
                if let Some(next) = step.next_space(start) {
                    *start = next;

                    Some(next)
                } else {
                    None
                }
            }),
        }
    }
}

impl<'ray> Iterator for Cast<'ray> {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
