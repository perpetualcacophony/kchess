pub mod cardinal;

use std::ops::{Add, Mul, Neg};

pub use cardinal::Cardinal;

mod diagonal;
pub use diagonal::Diagonal;

pub mod ray;
pub use ray::Ray;

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

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool;

    fn parse_step(step: Step) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    pub ranks: isize,
    pub files: isize,
}

impl Step {
    pub const fn new(ranks: isize, files: isize) -> Self {
        Self { ranks, files }
    }

    pub fn rotate_cw_once(self) -> Self {
        Self::new(self.files, self.ranks.neg())
    }

    pub fn rotate_cw(self, turns: usize) -> Self {
        (0..turns).fold(self, |step, _| step.rotate_cw_once())
    }

    pub const fn as_ne(&self) -> ((bool, usize), (bool, usize)) {
        (
            (self.ranks.is_positive(), self.ranks.unsigned_abs()),
            (self.files.is_positive(), self.files.unsigned_abs()),
        )
    }

    pub const fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        match cardinal {
            Cardinal::North => self.ranks.is_positive(),
            Cardinal::East => self.files.is_positive(),
            Cardinal::South => self.ranks.is_negative(),
            Cardinal::West => self.ranks.is_negative(),
        }
    }

    pub fn count_cardinal(&self, cardinal: Cardinal) -> Option<usize> {
        match cardinal {
            Cardinal::North => self
                .ranks
                .is_positive()
                .then_some(self.ranks.unsigned_abs()),
            Cardinal::East => self
                .files
                .is_positive()
                .then_some(self.files.unsigned_abs()),
            Cardinal::South => self
                .ranks
                .is_negative()
                .then_some(self.ranks.unsigned_abs()),
            Cardinal::West => self
                .files
                .is_negative()
                .then_some(self.ranks.unsigned_abs()),
        }
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

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self.direction.contains_cardinal(cardinal)
    }

    fn parse_step(step: Step) -> Option<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
