use crate::{
    direction::{Cardinal, Diagonal},
    Direction, UncheckedSpace,
};

pub fn moves(start: UncheckedSpace) -> [UncheckedSpace; 8] {
    [
        Cardinal::ARRAY.map(|cardinal| cardinal.next_space(start)),
        Diagonal::ARRAY.map(|diagonal| diagonal.next_space(start)),
    ]
    .concat()
    .try_into()
    .unwrap()
}
