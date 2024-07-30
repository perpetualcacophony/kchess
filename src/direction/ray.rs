use super::{Cardinal, Direction};
use crate::UncheckedSpace;

pub type RayStatic<Dir> = Ray<&'static Dir>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray<Dir = Direction> {
    limit: Option<usize>,
    direction: Dir,
}

impl<D> Ray<D> {
    pub const fn new(limit: Option<usize>, direction: D) -> Self {
        Self { limit, direction }
    }

    pub const fn no_limit(direction: D) -> Self {
        Self::new(None, direction)
    }

    pub const fn limited(limit: usize, direction: D) -> Self {
        Self::new(Some(limit), direction)
    }

    pub const fn once(direction: D) -> Self {
        Self::limited(1, direction)
    }

    pub fn iter(&self) -> Iter<D> {
        Iter::new(self)
    }
}

impl<C> Ray<Direction<C>>
where
    C: Into<Vec<Cardinal>>,
{
    pub fn into_vec(self) -> Ray {
        Ray {
            limit: self.limit,
            direction: self.direction.into_vec(),
        }
    }
}

impl<C> RayStatic<Direction<C>>
where
    C: Into<Vec<Cardinal>>,
    Direction<C>: Copy,
{
    pub fn into_vec(self) -> Ray {
        Ray {
            limit: self.limit,
            direction: (*self.direction).into_vec(),
        }
    }
}

impl<C> Ray<Direction<C>> {
    pub fn cast(&self, start: UncheckedSpace) -> impl Iterator<Item = UncheckedSpace> + '_
    where
        Self: Clone,
        for<'a> &'a C: IntoIterator<Item = &'a Cardinal>,
    {
        self.iter().scan(start, |start, dir| {
            *start = start.step(dir);

            Some(*start)
        })
    }
}

pub struct Iter<'ray, Dir> {
    limit: Option<usize>,
    direction: &'ray Dir,
}

impl<'ray, Dir> Iter<'ray, Dir> {
    fn new(ray: &'ray Ray<Dir>) -> Self {
        Self {
            limit: ray.limit,
            direction: &ray.direction,
        }
    }
}

impl<'ray, Dir> Iterator for Iter<'ray, Dir> {
    type Item = &'ray Dir;

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
