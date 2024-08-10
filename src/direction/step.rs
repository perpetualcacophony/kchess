use super::{Cardinal, Direction};
use crate::Space;
use std::ops::{Add, Mul, Neg};

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

    pub fn try_direction<D: Direction>(&self) -> Option<D> {
        D::parse_step(*self)
    }

    pub fn next_space(&self, start: &Space) -> Option<Space> {
        Some(Space::new(
            start.rank.checked_add_signed(self.ranks)?,
            start.file.checked_add_signed(self.files)?,
        ))
    }

    pub fn length(&self) -> f64 {
        let sum = (self.ranks.pow(2) + self.ranks.pow(2)) as f64;
        sum.sqrt()
    }

    pub fn divisible(self, rhs: Self) -> bool {
        self.ranks.checked_rem(rhs.ranks) == Some(0) && self.files.checked_rem(rhs.files) == Some(0)
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.ranks.checked_div(rhs.ranks)?,
            self.files.checked_div(rhs.files)?,
        ))
    }

    pub fn div_exact(self, rhs: Self) -> Option<isize> {
        self.divisible(rhs)
            .then(|| self.ranks.checked_div(rhs.ranks))
            .flatten()
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

impl Neg for Step {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(!self.ranks, !self.files)
    }
}

impl<'a, T: Direction> From<&'a T> for Step {
    fn from(value: &'a T) -> Self {
        value.as_step()
    }
}
