use crate::{
    direction::{ray::LimitedRay, Cardinal, Diagonal, InfiniteRay, Relative},
    ChessSide, UncheckedSpace,
};

pub fn captures(side: ChessSide, start: UncheckedSpace) -> [UncheckedSpace; 2] {
    [
        start.step(Relative::new(side, Diagonal::NORTHEAST)),
        start.step(Relative::new(side, Diagonal::NORTHWEST)),
    ]
}

pub fn steps(side: ChessSide, start: UncheckedSpace) -> [UncheckedSpace; 2] {
    let forward = Relative::new(side, Cardinal::NORTH);

    [start.step(forward), start.step(forward).step(forward)]
}

pub fn step_ray(side: ChessSide, moved: bool) -> LimitedRay<InfiniteRay<Relative>> {
    let limit = if moved { 1 } else { 2 };
    LimitedRay::new(Relative::forward(side), limit)
}

pub fn capture_rays(side: ChessSide) -> [LimitedRay<InfiniteRay<Relative<Diagonal>>>; 2] {
    [
        LimitedRay::new(Relative::new(side, Diagonal::NORTHEAST), 1),
        LimitedRay::new(Relative::new(side, Diagonal::NORTHWEST), 1),
    ]
}
