use crate::{ChessSide, UncheckedSpace};

use super::{Cardinal, Direction, DirectionBoxed, Direction};

pub type DirectionSlice<'a> = Direction<&'a [Cardinal]>;

impl<'a> DirectionSlice<'a> {
    pub fn into_boxed(self) -> DirectionBoxed {
        DirectionBoxed::from_cardinals(self.cardinals.into())
    }

    pub fn opposite(self) -> DirectionBoxed {
        self.map(|cardinals| cardinals.iter().copied().map(Cardinal::opposite).collect())
    }

    pub fn first(self) -> Option<Cardinal> {
        self.cardinals.first().copied()
    }

    pub fn align_cardinal(self, cardinal: Cardinal) -> DirectionBoxed {
        if let Some(first) = self.first() {
            let turns = first.turns_cw(cardinal);

            if turns == 0 {
                self.into_boxed()
            } else {
                self.map(|cardinals| {
                    cardinals
                        .iter()
                        .map(|cardinal| cardinal.rotate_cw(turns))
                        .collect()
                })
            }
        } else {
            self.into_boxed()
        }
    }

    pub fn relative(self, side: ChessSide) -> DirectionBoxed {
        self.align_cardinal(side.forward_cardinal())
    }
}

impl<'a> Direction for DirectionSlice<'a> {
    fn next_space(&self, start: UncheckedSpace) -> UncheckedSpace {
        self.into_iter().fold(start, UncheckedSpace::step_cardinal)
    }
}
