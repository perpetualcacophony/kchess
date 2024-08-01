use super::{DirectionExt, DirectionSlice};
use crate::UncheckedSpace;

mod builder;
pub use builder::RayBuilder;

pub mod set;
pub use set::RaySet;

pub type RayStatic = Ray<'static>;

pub struct Ray<'a> {
    limit: Option<usize>,
    direction: DirectionSlice<'a>,
}

impl<'a> Ray<'a> {
    pub const fn new(limit: Option<usize>, direction: DirectionSlice<'a>) -> Self {
        Self { limit, direction }
    }

    pub const fn from_builder(builder: RayBuilder<DirectionSlice<'a>>) -> Self {
        Self::new(builder.limit, builder.direction)
    }

    const fn steps(self) -> Steps<'a> {
        Steps::new(self)
    }

    pub fn cast(self, start: UncheckedSpace) -> Cast<'a> {
        Cast::new(self, start)
    }
}

struct Steps<'ray> {
    limit: Option<usize>,
    direction: DirectionSlice<'ray>,
}

impl<'ray> Steps<'ray> {
    const fn new(ray: Ray<'ray>) -> Self {
        Self {
            limit: ray.limit,
            direction: ray.direction,
        }
    }
}

impl<'ray> Iterator for Steps<'ray> {
    type Item = DirectionSlice<'ray>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit.as_mut() {
            if *limit == 0 {
                return None;
            } else {
                *limit -= 1;
            }
        }

        Some(self.direction)
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
    fn new(ray: Ray<'ray>, start: UncheckedSpace) -> Self {
        Self {
            inner: ray.steps().scan(start, |start, step| {
                *start = step.next_space(*start);

                Some(*start)
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
