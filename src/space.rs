use crate::{direction::Step, Direction};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Space {
    pub rank: usize,
    pub file: usize,
}

impl Space {
    pub fn new(rank: usize, file: usize) -> Self {
        Self { rank, file }
    }

    pub fn step(&self, direction: impl Direction) -> Self {
        direction.next_space(self).unwrap()
    }

    pub fn distance_step(&self, rhs: &Self) -> Option<Step> {
        let ranks = isize::try_from(rhs.rank).ok()? - isize::try_from(self.rank).ok()?;
        let files = isize::try_from(rhs.file).ok()? - isize::try_from(self.file).ok()?;
        Some(Step::new(ranks, files))
    }

    pub const fn rank(&self) -> usize {
        self.rank
    }

    pub const fn file(&self) -> usize {
        self.file
    }
}
