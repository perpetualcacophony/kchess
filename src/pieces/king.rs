use crate::{
    direction::{
        ray::{LimitedRay, Ray},
        Cardinal, Diagonal,
    },
    Direction, UncheckedSpace,
};

use super::queen::{self, QueenRay};

pub fn moves(start: UncheckedSpace) -> [UncheckedSpace; 8] {
    [
        Cardinal::ARRAY.map(|cardinal| cardinal.next_space(start)),
        Diagonal::ARRAY.map(|diagonal| diagonal.next_space(start)),
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub struct KingRay {
    inner: LimitedRay<QueenRay>,
}

impl KingRay {
    pub fn from_queen(ray: QueenRay) -> Self {
        Self {
            inner: ray.limited(1),
        }
    }
}

impl Ray for KingRay {
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace> {
        self.inner.next_space(space)
    }
}

pub fn rays() -> [KingRay; 8] {
    queen::rays().map(KingRay::from_queen)
}
