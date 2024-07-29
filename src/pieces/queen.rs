use crate::{direction::ray::Ray, UncheckedSpace};

use super::{
    bishop::{self, BishopRay},
    rook::{self, RookRay},
};

#[derive(Debug, Clone, Copy)]
pub struct QueenRay {
    kind: QueenRayKind,
}

impl QueenRay {
    pub fn bishop(ray: BishopRay) -> Self {
        Self {
            kind: QueenRayKind::Bishop(ray),
        }
    }

    pub fn rook(ray: RookRay) -> Self {
        Self {
            kind: QueenRayKind::Rook(ray),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum QueenRayKind {
    Bishop(BishopRay),
    Rook(RookRay),
}

impl Ray for QueenRay {
    fn next_space(&mut self, space: UncheckedSpace) -> Option<UncheckedSpace> {
        match self.kind {
            QueenRayKind::Bishop(ref mut ray) => ray.next_space(space),
            QueenRayKind::Rook(ref mut ray) => ray.next_space(space),
        }
    }
}

pub fn rays() -> [QueenRay; 8] {
    [
        bishop::rays().map(QueenRay::bishop),
        rook::rays().map(QueenRay::rook),
    ]
    .concat()
    .try_into()
    .unwrap()
}
