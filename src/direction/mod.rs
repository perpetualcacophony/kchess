pub mod cardinal;

use std::ops::{Add, Mul, Neg};

pub use cardinal::Cardinal;

mod diagonal;
pub use diagonal::Diagonal;

pub mod ray;

use crate::{ChessSide, UncheckedSpace};

pub trait Direction {
    fn as_step(&self) -> Step;

    fn next_space(&self, start: UncheckedSpace) -> Option<UncheckedSpace> {
        self.as_step().next_space(start)
    }

    fn relative(self, side: ChessSide) -> Relative<Self>
    where
        Self: Sized,
    {
        Relative::new(self, side)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    ranks: isize,
    files: isize,
}

impl Step {
    pub fn new(ranks: isize, files: isize) -> Self {
        Self { ranks, files }
    }

    pub fn rotate_cw_once(self) -> Self {
        Self::new(self.files, self.ranks.neg())
    }

    pub fn rotate_cw(self, turns: usize) -> Self {
        (0..turns).fold(self, |step, _| step.rotate_cw_once())
    }
}

impl Step {
    pub fn next_space(&self, start: UncheckedSpace) -> Option<UncheckedSpace> {
        Some(UncheckedSpace::new(
            start.rank.checked_add_signed(self.ranks)?,
            start.file.checked_add_signed(self.files)?,
        ))
    }
}

impl Add for Step {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.ranks + rhs.ranks, self.files + rhs.files)
    }
}

impl Mul<isize> for Step {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.ranks * rhs, self.files * rhs)
    }
}

pub struct Relative<D> {
    direction: D,
    side: ChessSide,
}

impl<D> Relative<D> {
    pub fn new(direction: D, side: ChessSide) -> Self {
        Self { direction, side }
    }
}

impl<D: Direction> Direction for Relative<D> {
    fn as_step(&self) -> Step {
        self.direction
            .as_step()
            .rotate_cw(Cardinal::NORTH.turns_cw(self.side.forward_cardinal()))
    }
}
