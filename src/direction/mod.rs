pub mod cardinal;

pub use cardinal::Cardinal;

pub mod diagonal;

pub mod relative;
pub use relative::Relative;

pub mod ray;
pub use ray::Ray;

use crate::{ChessSide, UncheckedSpace};

pub trait Direction {
    fn opposite(self) -> Self;
    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace;
    fn perpendicular(self) -> [Self; 2]
    where
        Self: std::marker::Sized;
}

#[derive(Clone, Copy, Debug)]
pub enum DirectionUnion<A, B> {
    A(A),
    B(B),
}

impl<A, B> Direction for DirectionUnion<A, B>
where
    A: Direction,
    B: Direction,
{
    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        match self {
            Self::A(direction) => direction.next_space(start),
            Self::B(direction) => direction.next_space(start),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::A(direction) => Self::A(direction.opposite()),
            Self::B(direction) => Self::B(direction.opposite()),
        }
    }

    fn perpendicular(self) -> [Self; 2]
    where
        Self: std::marker::Sized,
    {
        match self {
            Self::A(direction) => direction.perpendicular().map(Self::A),
            Self::B(direction) => direction.perpendicular().map(Self::B),
        }
    }
}

pub type DirectionSingle = DirectionArray<1>;

pub type DirectionArray<const N: usize> = DirectionStruct<[Cardinal; N]>;

impl DirectionArray<2> {
    pub const fn double(a: Cardinal, b: Cardinal) -> Self {
        Self::new([a, b])
    }
}

impl<const N: usize> DirectionArray<N> {
    pub fn relative(&self, side: ChessSide) -> Self
    where
        Self: Clone,
    {
        if side == ChessSide::White {
            *self
        } else {
            Self::new(self.cardinals.map(Cardinal::opposite))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DirectionStruct<Collection = Vec<Cardinal>> {
    cardinals: Collection,
}

impl<C> DirectionStruct<C> {
    pub const fn new(cardinals: C) -> Self {
        Self { cardinals }
    }

    pub fn into_vec(self) -> DirectionStruct<Vec<Cardinal>>
    where
        C: Into<Vec<Cardinal>>,
    {
        DirectionStruct::new(self.cardinals.into())
    }

    pub fn map<C2, F>(self, f: F) -> DirectionStruct<C2>
    where
        F: FnOnce(C) -> C2,
    {
        DirectionStruct::new(f(self.cardinals))
    }
}

impl<'a, C: 'a> DirectionStruct<C>
where
    &'a C: IntoIterator<Item = &'a Cardinal>,
{
    fn iter(&'a self) -> Iter<'a, C> {
        (&self.cardinals).into_iter().copied()
    }

    pub fn next_space(&'a self, start: UncheckedSpace) -> UncheckedSpace {
        self.iter().fold(start, UncheckedSpace::cardinal)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&'a self) -> usize
    where
        for<'b> Iter<'b, C>: ExactSizeIterator,
    {
        self.iter().len()
    }
}

impl<C> DirectionStruct<C>
where
    Self: FromIterator<Cardinal>,
{
    pub fn from_array<const LEN: usize>(array: [Cardinal; LEN]) -> Self {
        array.into_iter().collect()
    }

    pub fn single(cardinal: Cardinal) -> Self {
        Self::from_array([cardinal])
    }

    pub fn double(a: Cardinal, b: Cardinal) -> Self {
        Self::from_array([a, b])
    }

    pub fn triple(a: Cardinal, b: Cardinal, c: Cardinal) -> Self {
        Self::from_array([a, b, c])
    }
}

impl<'a, C: 'a> DirectionStruct<C>
where
    Self: FromIterator<Cardinal>,
    &'a C: IntoIterator<Item = &'a Cardinal>,
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

pub type Iter<'a, I> = std::iter::Copied<<&'a I as IntoIterator>::IntoIter>;

impl<C> FromIterator<Cardinal> for DirectionStruct<C>
where
    C: FromIterator<Cardinal>,
{
    fn from_iter<T: IntoIterator<Item = Cardinal>>(iter: T) -> Self {
        Self::new(C::from_iter(iter))
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
