use crate::{ChessSide, UncheckedSpace};

use super::{Cardinal, Direction, DirectionBoxed, DirectionExt};

pub type DirectionSlice<'a> = Direction<&'a [Cardinal]>;

impl<'a> DirectionSlice<'a> {
    pub fn into_boxed(self) -> DirectionBoxed {
        DirectionBoxed::from_cardinals(self.cardinals.into())
    }

    pub fn opposite(self) -> DirectionBoxed {
        self.map(|cardinals| cardinals.iter().copied().map(Cardinal::opposite).collect())
    }

    pub fn relative(self, side: ChessSide) -> DirectionBoxed {
        if side == ChessSide::White {
            self.into_boxed()
        } else {
            self.opposite()
        }
    }
}

impl<'a> DirectionExt for DirectionSlice<'a> {
    fn next_space(&self, start: UncheckedSpace) -> UncheckedSpace {
        self.into_iter().fold(start, UncheckedSpace::step_cardinal)
    }
}
