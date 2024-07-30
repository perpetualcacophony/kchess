use crate::{
    direction::{Cardinal, DirectionStruct},
    Direction,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Space {
    rank: usize,
    file: usize,
}

impl Space {
    pub const fn rank(self) -> usize {
        self.rank
    }

    pub const fn file(self) -> usize {
        self.file
    }

    pub(super) unsafe fn new_unchecked(rank: usize, file: usize) -> Self {
        Self::from_unchecked(UncheckedSpace::new(rank, file))
    }

    unsafe fn from_unchecked(unchecked: UncheckedSpace) -> Self {
        Self {
            rank: unchecked.rank,
            file: unchecked.file,
        }
    }

    pub fn as_unchecked(self) -> UncheckedSpace {
        UncheckedSpace {
            rank: self.rank,
            file: self.file,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UncheckedSpace {
    pub rank: usize,
    pub file: usize,
}

impl UncheckedSpace {
    pub fn new(rank: usize, file: usize) -> Self {
        Self { rank, file }
    }

    pub(super) fn cardinal(mut self, cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => self.file += 1,
            Cardinal::East => self.rank += 1,
            Cardinal::South => self.file -= 1,
            Cardinal::West => self.rank -= 1,
        };

        self
    }

    pub fn step<'a, D>(self, direction: &'a DirectionStruct<D>) -> Self
    where
        &'a D: IntoIterator<Item = &'a Cardinal>,
    {
        direction.next_space(self)
    }

    pub fn step_in_place<'a, D>(&mut self, direction: &'a DirectionStruct<D>)
    where
        &'a D: IntoIterator<Item = &'a Cardinal>,
    {
        *self = self.step(direction)
    }
}
