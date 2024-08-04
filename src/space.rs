use crate::direction::{Direction, Step};

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

    pub fn step(self, direction: impl Direction) -> Self {
        direction.next_space(self).unwrap()
    }

    pub fn distance_step(self, rhs: Self) -> Option<Step> {
        let ranks = isize::try_from(rhs.rank).ok()? - isize::try_from(self.rank).ok()?;
        let files = isize::try_from(rhs.file).ok()? - isize::try_from(self.file).ok()?;
        Some(Step::new(ranks, files))
    }
}
