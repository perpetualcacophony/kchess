use crate::{
    direction::{ray::Ray, Cardinal, Diagonal},
    Direction, UncheckedSpace,
};

use super::queen::{self, QueenDirection};

pub fn moves(start: UncheckedSpace) -> [UncheckedSpace; 8] {
    [
        Cardinal::ARRAY.map(|cardinal| cardinal.next_space(start)),
        Diagonal::ARRAY.map(|diagonal| diagonal.next_space(start)),
    ]
    .concat()
    .try_into()
    .unwrap()
}

pub type KingRay = Ray<QueenDirection>;

pub fn rays() -> [KingRay; 8] {
    queen::directions().map(KingRay::once)
}
