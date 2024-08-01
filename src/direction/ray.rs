use super::Step;
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

pub mod set;
pub use set::RaySet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ray {
    limit: Option<usize>,
    step: Step,
}

impl Ray {
    pub const fn new(limit: Option<usize>, step: Step) -> Self {
        Self { limit, step }
    }

    pub const fn from_builder(builder: RayBuilder) -> Self {
        Self::new(builder.limit, builder.step)
    }

    const fn steps(&self) -> Steps {
        Steps::new(self)
    }

    pub fn cast(&self, start: UncheckedSpace) -> Cast {
        Cast::new(self, start)
    }

    pub fn to_builder(self) -> RayBuilder {
        RayBuilder {
            step: self.step,
            limit: self.limit,
        }
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

type CastInner<'ray> = std::iter::Scan<
    Steps<'ray>,
    UncheckedSpace,
    fn(&mut UncheckedSpace, <Steps as Iterator>::Item) -> Option<UncheckedSpace>,
>;

pub struct Cast<'ray> {
    inner: CastInner<'ray>,
}

impl<'ray> Cast<'ray> {
    fn new(ray: &'ray Ray, start: UncheckedSpace) -> Self {
        Self {
            inner: ray.steps().scan(start, |start, step| {
                if let Some(next) = step.next_space(*start) {
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
    type Item = UncheckedSpace;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
