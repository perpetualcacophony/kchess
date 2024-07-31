use crate::{
    direction::{self, ray::RayOwned, Cardinal, DirectionArray, DirectionCardinal},
    ChessSide,
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

pub fn step_ray(side: ChessSide, moved: bool) -> RayOwned {
    let limit = if moved { 1 } else { 2 };
    RayOwned::new(
        Some(limit),
        direction::cardinal::NORTH.relative(side).into_owned(),
    )
}

pub fn capture_rays(side: ChessSide) -> [RayOwned; 2] {
    [
        RayOwned::new(
            Some(1),
            direction::diagonal::NORTHEAST.relative(side).into_owned(),
        ),
        RayOwned::new(
            Some(1),
            direction::diagonal::NORTHWEST.relative(side).into_owned(),
        ),
    ]
}
