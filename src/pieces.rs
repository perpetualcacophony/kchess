use crate::{direction::ray::RaySet, ChessSide};

pub mod pawn;

pub mod knight;

pub mod bishop;

pub mod rook;

pub mod queen;

pub mod king;

#[derive(Clone, Debug, PartialEq)]
pub struct ChessPiece {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: RaySet,
    pub capture_rays: Option<RaySet>,
}

impl ChessPiece {
    pub fn pawn(side: ChessSide, moved: bool) -> ChessPiece {
        ChessPiece {
            value: 1,
            can_promote: true,
            valid_promotion: false,
            checkmate_possible: false,
            rays: pawn::step_ray(side, moved),
            capture_rays: Some(pawn::capture_rays(side)),
        }
    }

    pub fn knight() -> ChessPiece {
        ChessPiece {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: knight::rays(),
            capture_rays: None,
        }
    }

    pub fn bishop() -> ChessPiece {
        ChessPiece {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: bishop::rays(),
            capture_rays: None,
        }
    }

    pub fn rook() -> ChessPiece {
        ChessPiece {
            value: 5,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: rook::rays(),
            capture_rays: None,
        }
    }

    pub fn queen() -> ChessPiece {
        ChessPiece {
            value: 9,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: queen::rays(),
            capture_rays: None,
        }
    }

    pub fn king() -> ChessPiece {
        ChessPiece {
            value: usize::MAX,
            can_promote: false,
            valid_promotion: false,
            checkmate_possible: true,
            rays: king::rays(),
            capture_rays: None,
        }
    }
}
