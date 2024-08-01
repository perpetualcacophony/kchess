use crate::direction::ray::{RayOwned, RaySet};

use super::queen;

pub type KingRay = RayOwned;

pub fn rays() -> RaySet {
    queen::rays().map(|_| Some(1))
}
