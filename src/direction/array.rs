use super::{Cardinal, Direction, DirectionBoxed, DirectionExt, DirectionSlice};

pub type DirectionArray<const N: usize> = Direction<[Cardinal; N]>;

impl<const N: usize> DirectionArray<N> {
    pub fn as_slice(&self) -> DirectionSlice<'_> {
        DirectionSlice::from_cardinals(self.cardinals.as_slice())
    }

    pub fn boxed(&self) -> DirectionBoxed {
        DirectionBoxed::from_cardinals(self.cardinals.into())
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

impl<const N: usize> DirectionExt for DirectionArray<N> {
    fn next_space(&self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        self.as_slice().next_space(start)
    }
}
