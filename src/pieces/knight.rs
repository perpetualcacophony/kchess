use crate::{
    direction::{
        ray::{LimitedRay, Ray},
        Cardinal, InfiniteRay,
    },
    Direction, UncheckedSpace,
};

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

impl Direction for KnightMove {
    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace {
        start.step(self.long).step(self.long).step(self.short)
    }

    fn opposite(self) -> Self {
        Self::new(!self.long, !self.short)
    }

    fn perpendicular(self) -> [Self; 2] {
        todo!()
    }
}

pub struct KnightRay {
    inner: LimitedRay<InfiniteRay<KnightMove>>,
}

impl KnightRay {
    pub const fn new(direction: KnightMove) -> Self {
        Self {
            inner: LimitedRay::new(direction, 1),
        }
    }
}

impl Ray for KnightRay {
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace> {
        self.inner.next_space(space)
    }
}

pub fn rays() -> [KnightRay; 8] {
    let moves: [KnightMove; 8] = Cardinal::ARRAY
        .map(KnightMove::from_long)
        .concat()
        .try_into()
        .unwrap();

    moves.map(KnightRay::new)
}
