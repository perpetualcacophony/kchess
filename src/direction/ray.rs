use super::Direction;
use crate::UncheckedSpace;

#[derive(Copy, Clone, Debug)]
pub struct InfiniteRay<Dir> {
    direction: Dir,
}

impl<Dir> InfiniteRay<Dir> {
    pub const fn new(direction: Dir) -> Self {
        Self { direction }
    }

    pub fn map_array<const N: usize>(array: [Dir; N]) -> [Self; N] {
        array.map(Self::new)
    }

    pub fn limited(self, limit: usize) -> LimitedRay<Dir> {
        LimitedRay { limit, inner: self }
    }
}

pub trait Ray {
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace>;

    fn cast<'a>(self, start: UncheckedSpace) -> IntoIter<'a>
    where
        Self: Sized + 'a,
    {
        IntoIter {
            space: start,
            ray: Box::new(self),
        }
    }

    fn boxed<'a>(self) -> Box<dyn Ray + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<Dir> Ray for InfiniteRay<Dir>
where
    Dir: Direction,
{
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace> {
        Some(space.step(self.direction))
    }
}

pub struct IntoIter<'a> {
    space: UncheckedSpace,
    ray: Box<dyn Ray + 'a>,
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = UncheckedSpace;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.ray.next_space(self.space);

        if let Some(next_space) = item {
            self.space = next_space;
        }

        item
    }
}

pub struct LimitedRay<Dir> {
    inner: InfiniteRay<Dir>,
    limit: usize,
}

impl<Dir> LimitedRay<Dir> {
    pub const fn new(direction: Dir, limit: usize) -> Self {
        Self {
            inner: InfiniteRay::new(direction),
            limit,
        }
    }
}

impl<Dir> Ray for LimitedRay<Dir>
where
    Dir: Direction,
{
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace> {
        if self.limit == 0 {
            None
        } else {
            self.inner.next_space(space)
        }
    }
}
