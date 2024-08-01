pub mod cardinal;

pub use cardinal::Cardinal;

pub mod diagonal;

pub mod ray;

use crate::{ChessSide, UncheckedSpace};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Direction<Cardinals> {
    cardinals: Cardinals,
}

pub type DirectionBoxed = Direction<Box<[Cardinal]>>;

pub type DirectionArray<const N: usize> = Direction<[Cardinal; N]>;

impl<const N: usize> DirectionArray<N> {
    pub fn as_slice(&self) -> DirectionSlice<'_> {
        DirectionSlice::from_cardinals(self.cardinals.as_slice())
    }
}

pub type DirectionSlice<'a> = Direction<&'a [Cardinal]>;

impl DirectionSlice<'_> {
    pub fn into_boxed(self) -> DirectionBoxed {
        DirectionBoxed::from_cardinals(self.cardinals.into())
    }

    pub fn opposite(self) -> DirectionBoxed {
        self.map(|cardinals| {
            cardinals
                .into_iter()
                .copied()
                .map(Cardinal::opposite)
                .collect()
        })
    }

    pub fn relative(self, side: ChessSide) -> DirectionBoxed {
        if side == ChessSide::White {
            self.into_boxed()
        } else {
            self.opposite()
        }
    }

    pub fn next_space(self, start: UncheckedSpace) -> UncheckedSpace {
        self.into_iter().fold(start, UncheckedSpace::cardinal)
    }
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

impl<'a> IntoIterator for &'a DirectionBoxed {
    type Item = Cardinal;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Cardinal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cardinals.iter().copied()
    }
}

impl<'a> IntoIterator for DirectionSlice<'a> {
    type Item = Cardinal;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Cardinal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cardinals.iter().copied()
    }
}

impl DirectionBoxed {
    pub const fn as_ref(&self) -> DirectionSlice {
        DirectionSlice::from_cardinals(&self.cardinals)
    }
}

impl<'a, Cardinals> Direction<Cardinals>
where
    &'a Self: IntoIterator<Item = Cardinal> + 'a,
{
    pub fn iter(&'a self) -> impl Iterator<Item = Cardinal> + 'a {
        self.into_iter()
    }

    pub fn next_space(&'a self, start: UncheckedSpace) -> UncheckedSpace {
        self.iter().fold(start, UncheckedSpace::cardinal)
    }
}

impl<'a, Cardinals> Direction<Cardinals>
where
    Self: FromIterator<Cardinal>,
    &'a Self: IntoIterator<Item = Cardinal> + 'a,
{
    pub fn opposite(&'a self) -> Self {
        Self::from_iter(self.iter().map(Cardinal::opposite))
    }

    pub fn relative(&'a self, side: ChessSide) -> Self
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

impl Direction<OneOrTwo<Cardinal>> {
    pub fn as_slice(&self) -> DirectionSlice {
        DirectionSlice::from_cardinals(self.cardinals.as_slice())
    }

    pub fn into_boxed(self) -> DirectionBoxed {
        self.as_slice().into_boxed()
    }
}
