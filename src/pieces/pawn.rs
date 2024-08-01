use crate::{
    direction::{
        ray::{RayBuilder, RaySet},
        Cardinal, Diagonal,
    },
    ChessSide, Direction,
};

pub fn step_ray(side: ChessSide, moved: bool) -> RaySet {
    let limit = if moved { 1 } else { 2 };
    RaySet::new().with(RayBuilder::new(Cardinal::NORTH.relative(side)).limit(limit))
}

pub fn capture_rays(side: ChessSide) -> RaySet {
    RaySet::new()
        .with(RayBuilder::new(Diagonal::NORTHEAST.relative(side)).once())
        .with(RayBuilder::new(Diagonal::NORTHWEST.relative(side)).once())
}
