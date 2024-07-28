use crate::ChessSide;

use super::{Cardinal, Direction};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Relative<Direction = Cardinal> {
    side: ChessSide,
    inner: Direction,
}

impl<Direction> Relative<Direction> {
    pub const fn new(side: ChessSide, inner: Direction) -> Self {
        Self { side, inner }
    }

    pub fn as_inner(self) -> Direction
    where
        Direction: std::ops::Not<Output = Direction>,
    {
        match self.side {
            ChessSide::White => self.inner,
            ChessSide::Black => !self.inner,
        }
    }
}

impl Relative<Cardinal> {
    pub const fn forward(side: ChessSide) -> Self {
        Self::new(side, Cardinal::NORTH)
    }

    pub const fn backward(side: ChessSide) -> Self {
        Self::new(side, Cardinal::SOUTH)
    }

    pub const fn left(side: ChessSide) -> Self {
        Self::new(side, Cardinal::WEST)
    }

    pub const fn right(side: ChessSide) -> Self {
        Self::new(side, Cardinal::EAST)
    }

    pub fn as_cardinal(self) -> Cardinal {
        match self.side {
            ChessSide::White => self.inner,
            ChessSide::Black => !self.inner,
        }
    }
}

impl<T> Direction for Relative<T>
where
    T: Direction + std::ops::Not<Output = T>,
{
    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        self.as_inner().next_space(start)
    }

    fn opposite(self) -> Self {
        Self {
            side: self.side,
            inner: self.inner.opposite(),
        }
    }

    fn perpendicular(self) -> [Self; 2] {
        self.inner.perpendicular().map(|inner| Self {
            side: self.side,
            inner,
        })
    }
}
