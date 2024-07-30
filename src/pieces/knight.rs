use crate::{
    direction::{cardinal, ray::Ray, Cardinal, DirectionArray},
    Direction, UncheckedSpace,
};

/* #[derive(Copy, Debug, Clone)]
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

    pub fn opposite(&self) -> Self {
        Self::new(!self.long, !self.short)
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
        Self::opposite(&self)
    }

    fn perpendicular(self) -> [Self; 2] {
        todo!()
    }
}
 */
pub fn from_long(long: Cardinal) -> [DirectionArray<3>; 2] {
    long.perpendicular().map(|cardinal| new(long, cardinal))
}

pub fn new(long: Cardinal, short: Cardinal) -> DirectionArray<3> {
    assert!(long.perpendicular_to(short));

    DirectionArray::new([long, long, short])
}

pub fn directions() -> [DirectionArray<3>; 8] {
    Cardinal::ARRAY
        .map(|cardinal| from_long(cardinal))
        .concat()
        .try_into()
        .unwrap()
}

pub type KnightRay = Ray<DirectionArray<3>>;

pub fn rays() -> [KnightRay; 8] {
    directions().map(KnightRay::once)
}
