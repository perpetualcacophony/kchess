use crate::{direction::Cardinal, Direction, UncheckedSpace};

#[derive(Copy, Debug, Clone)]
pub struct KnightMove {
    long: Cardinal,
    short: Cardinal,
}

impl KnightMove {
    pub fn new(long: Cardinal, short: Cardinal) -> Self {
        assert!(long.perpendicular().contains(&short));

        Self { long, short }
    }

    pub fn from_long(long: Cardinal) -> [Self; 2] {
        long.perpendicular().map(|short| Self::new(long, short))
    }

    pub fn next_space(self, start: UncheckedSpace) -> UncheckedSpace {
        start.step(self.long).step(self.long).step(self.short)
    }
}

pub fn moves(start: UncheckedSpace) -> [UncheckedSpace; 8] {
    Cardinal::ARRAY
        .map(|cardinal| KnightMove::from_long(cardinal).map(|knight| knight.next_space(start)))
        .concat()
        .try_into()
        .unwrap()
}
