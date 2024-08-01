use crate::{
    direction::{
        self,
        ray::{RayBuilder, RaySet},
    },
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

pub fn step_ray(side: ChessSide, moved: bool) -> RaySet {
    let limit = if moved { 1 } else { 2 };
    RaySet::new().with(RayBuilder::new(direction::cardinal::NORTH.relative(side)).limit(limit))
}

pub fn capture_rays(side: ChessSide) -> RaySet {
    RaySet::new()
        .with(RayBuilder::new(direction::diagonal::NORTHEAST.relative(side)).limit(1))
        .with(RayBuilder::new(direction::diagonal::NORTHWEST.relative(side)).limit(1))
}
