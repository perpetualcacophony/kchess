use crate::direction::ray::RaySet;

use super::{bishop, rook};

pub fn rays() -> RaySet {
    RaySet::new()
        .with_set(bishop::rays())
        .with_set(rook::rays())
}
