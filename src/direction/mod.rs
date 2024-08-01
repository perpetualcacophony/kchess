pub mod cardinal;

pub use cardinal::Cardinal;

pub mod diagonal;

pub mod ray;

pub mod slice;
pub use slice::DirectionSlice;

pub mod boxed;
pub use boxed::DirectionBoxed;

pub mod array;
pub use array::DirectionArray;

use crate::UncheckedSpace;

pub trait DirectionExt {
    fn next_space(&self, start: UncheckedSpace) -> UncheckedSpace;
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Direction<Cardinals> {
    cardinals: Cardinals,
}

impl<Cardinals> Direction<Cardinals> {
    pub const fn from_cardinals(cardinals: Cardinals) -> Self {
        Self { cardinals }
    }

    pub fn map<NewCardinals, F>(self, mut f: F) -> Direction<NewCardinals>
    where
        F: FnMut(Cardinals) -> NewCardinals,
    {
        Direction::from_cardinals(f(self.cardinals))
    }
}

impl<'a> IntoIterator for DirectionSlice<'a> {
    type Item = Cardinal;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Cardinal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cardinals.iter().copied()
    }
}

#[derive(Clone, Debug, Copy)]
pub enum OneOrTwo<T> {
    One([T; 1]),
    Two([T; 2]),
}

impl<T> OneOrTwo<T> {
    fn as_slice(&self) -> &[T] {
        match self {
            Self::One(array) => array,
            Self::Two(array) => array,
        }
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.as_slice().iter()
    }
}

impl<'a, T: 'a> IntoIterator for &'a OneOrTwo<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone> From<OneOrTwo<T>> for Vec<T> {
    fn from(value: OneOrTwo<T>) -> Self {
        match value {
            OneOrTwo::One(array) => array.to_vec(),
            OneOrTwo::Two(array) => array.to_vec(),
        }
    }
}

pub type DirectionCardinal = DirectionArray<1>;
pub type DirectionDiagonal = DirectionArray<2>;

impl DirectionDiagonal {
    pub const fn diagonal(a: Cardinal, b: Cardinal) -> Self {
        diagonal::new(a, b)
    }

    pub const fn diagonal_ne(north: bool, east: bool) -> Self {
        diagonal::new_ne(north, east)
    }
}

impl Direction<OneOrTwo<Cardinal>> {
    pub fn as_slice(&self) -> DirectionSlice {
        DirectionSlice::from_cardinals(self.cardinals.as_slice())
    }

    pub fn into_boxed(self) -> DirectionBoxed {
        self.as_slice().into_boxed()
    }
}
