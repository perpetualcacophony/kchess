use super::Direction;
use crate::UncheckedSpace;

#[derive(Copy, Clone, Debug)]
pub struct Ray<Dir> {
    direction: Dir,
    space: UncheckedSpace,
}

impl<Dir> Ray<Dir> {
    pub const fn new(start: UncheckedSpace, direction: Dir) -> Self {
        Self {
            direction,
            space: start,
        }
    }

    pub fn map_iter(
        start: UncheckedSpace,
        directions: impl IntoIterator<Item = Dir>,
    ) -> impl Iterator<Item = Self> {
        directions
            .into_iter()
            .map(move |direction| Self::new(start, direction))
    }

    pub fn map_array<const N: usize>(start: UncheckedSpace, array: [Dir; N]) -> [Self; N] {
        array.map(|direction| Self::new(start, direction))
    }

    pub fn boxed_iter(self) -> Box<dyn Iterator<Item = UncheckedSpace>>
    where
        Dir: Direction + 'static,
    {
        Box::new(self)
    }
}

impl<Dir> Iterator for Ray<Dir>
where
    Dir: Direction,
{
    type Item = UncheckedSpace;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.space;
        self.space.step_in_place(self.direction);
        Some(item)
    }
}
