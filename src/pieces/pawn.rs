use crate::{
    direction::{
        self,
        ray::{Ray, RayStatic},
        Cardinal, Diagonal, DirectionArray, DirectionSingle, DirectionStruct, OneOrTwo, Relative,
    },
    ChessSide, UncheckedSpace,
};

/* pub fn captures(side: ChessSide, start: UncheckedSpace) -> [UncheckedSpace; 2] {
    [
        start.step(Relative::new(side, Diagonal::NORTHEAST)),
        start.step(Relative::new(side, Diagonal::NORTHWEST)),
    ]
}

pub fn steps(side: ChessSide, start: UncheckedSpace) -> [UncheckedSpace; 2] {
    let forward = Relative::new(side, Cardinal::NORTH);

    [start.step(forward), start.step(forward).step(forward)]
} */

pub fn step_ray(side: ChessSide, moved: bool) -> Ray<DirectionSingle> {
    let limit = if moved { 1 } else { 2 };
    Ray::limited(limit, direction::cardinal::NORTH.relative(side))
}

pub fn capture_rays(side: ChessSide) -> [Ray<DirectionArray<2>>; 2] {
    [
        Ray::once(direction::diagonal::NORTHEAST.relative(side)),
        Ray::once(direction::diagonal::NORTHWEST.relative(side)),
    ]
}
