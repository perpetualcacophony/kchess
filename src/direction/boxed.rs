use super::{Cardinal, Direction, DirectionExt, DirectionSlice};

pub type DirectionBoxed = Direction<Box<[Cardinal]>>;

impl<'a> IntoIterator for &'a DirectionBoxed {
    type Item = Cardinal;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Cardinal>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cardinals.iter().copied()
    }
}

impl DirectionBoxed {
    pub const fn as_slice(&self) -> DirectionSlice {
        DirectionSlice::from_cardinals(&self.cardinals)
    }
}

impl DirectionExt for DirectionBoxed {
    fn next_space(&self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        self.as_slice().next_space(start)
    }
}
