use crate::UncheckedSpace;

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

    pub fn boxed_iter(self) -> Box<dyn Iterator<Item = UncheckedSpace>> {
        Box::new(self)
    }
}

#[derive(Debug, Clone, Copy)]
enum QueenRayKind {
    Bishop(BishopRay),
    Rook(RookRay),
}

impl Iterator for QueenRay {
    type Item = UncheckedSpace;

    fn next(&mut self) -> Option<Self::Item> {
        match self.kind {
            QueenRayKind::Bishop(ref mut ray) => ray.next(),
            QueenRayKind::Rook(ref mut ray) => ray.next(),
        }
    }
}

pub fn rays(start: UncheckedSpace) -> [QueenRay; 8] {
    [
        bishop::rays(start).map(QueenRay::bishop),
        rook::rays(start).map(QueenRay::rook),
    ]
    .concat()
    .try_into()
    .unwrap()
}
