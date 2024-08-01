use crate::direction::ray::RaySet;

use super::queen;

pub fn rays() -> RaySet {
    queen::rays().map(|_| Some(1))
}
