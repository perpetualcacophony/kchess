pub mod cardinal;

pub use cardinal::Cardinal;

pub mod diagonal;

pub mod ray;

use crate::{ChessSide, UncheckedSpace};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Direction<Cardinals = Vec<Cardinal>> {
    cardinals: Cardinals,
}

impl<Cardinals> Direction<Cardinals> {
    pub const fn from_cardinals(cardinals: Cardinals) -> Self {
        Self { cardinals }
    }

    pub fn into_owned(self) -> Direction
    where
        Cardinals: Into<Vec<Cardinal>>,
    {
        Direction::from_cardinals(self.cardinals.into())
    }

    pub fn map<NewCardinals, F>(self, mut f: F) -> Direction<NewCardinals>
    where
        F: FnMut(Cardinals) -> NewCardinals,
    {
        Direction::from_cardinals(f(self.cardinals))
    }
}

impl<Cardinals> Direction<Cardinals>
where
    for<'a> &'a Self: IntoIterator<Item = Cardinal>,
{
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn next_space(&self, start: UncheckedSpace) -> UncheckedSpace {
        self.iter().fold(start, UncheckedSpace::cardinal)
    }
}

impl<Cardinals> Direction<Cardinals>
where
    Self: FromIterator<Cardinal>,
    for<'a> &'a Self: IntoIterator<Item = Cardinal>,
{
    pub fn opposite(&self) -> Self {
        Self::from_iter(self.iter().map(Cardinal::opposite))
    }

    pub fn relative(&self, side: ChessSide) -> Self
    where
        Self: Clone,
    {
        if side == ChessSide::White {
            self.clone()
        } else {
            self.opposite()
        }
    }
}

impl<'a, Cardinals> IntoIterator for &'a Direction<Cardinals>
where
    &'a Cardinals: IntoIterator<Item = &'a Cardinal>,
{
    type Item = Cardinal;
    type IntoIter = Iter<'a, Cardinals>;

    fn into_iter(self) -> Self::IntoIter {
        self.cardinals.into_iter().copied()
    }
}

pub type Iter<'a, Cardinals> = std::iter::Copied<<&'a Cardinals as IntoIterator>::IntoIter>;

impl<Cardinals> FromIterator<Cardinal> for Direction<Cardinals>
where
    Cardinals: FromIterator<Cardinal>,
{
    fn from_iter<T: IntoIterator<Item = Cardinal>>(iter: T) -> Self {
        Self::from_cardinals(Cardinals::from_iter(iter))
    }
}

#[derive(Clone, Debug, Copy)]
pub enum OneOrTwo<T> {
    One([T; 1]),
    Two([T; 2]),
}

impl<T> OneOrTwo<T> {
    fn iter(&self) -> std::slice::Iter<T> {
        match self {
            Self::One(array) => array.iter(),
            Self::Two(array) => array.iter(),
        }
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

pub type DirectionArray<const N: usize> = Direction<[Cardinal; N]>;

impl<const N: usize> DirectionArray<N> {
    pub fn opposite(self) -> Self {
        self.map(|cardinals| cardinals.map(Cardinal::opposite))
    }

    pub fn relative(self, side: ChessSide) -> Self {
        if side == ChessSide::White {
            self
        } else {
            self.opposite()
        }
    }
}

impl DirectionArray<1> {
    pub const fn new(cardinal: Cardinal) -> Self {
        Self::from_cardinals([cardinal])
    }
}

impl DirectionArray<2> {
    pub const fn new(a: Cardinal, b: Cardinal) -> Self {
        Self::from_cardinals([a, b])
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
