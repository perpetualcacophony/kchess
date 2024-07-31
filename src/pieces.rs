use crate::{
    direction::ray::{RayOwned as Ray, RayStatic},
    ChessSide,
};

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
    pub rays: Vec<Ray>,
    pub capture_rays: Option<Vec<Ray>>,
}

impl ChessPiece {
    pub fn pawn(side: ChessSide, moved: bool) -> ChessPiece {
        ChessPiece {
            value: 1,
            can_promote: true,
            valid_promotion: false,
            checkmate_possible: false,
            rays: vec![pawn::step_ray(side, moved)],
            capture_rays: Some(pawn::capture_rays(side).to_vec()),
        }
    }

    pub fn knight() -> ChessPiece {
        ChessPiece {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: knight::rays().to_vec(),
            capture_rays: None,
        }
    }

    pub fn bishop() -> ChessPiece {
        ChessPiece {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: bishop::rays().map(RayStatic::into_owned).to_vec(),
            capture_rays: None,
        }
    }

    pub fn rook() -> ChessPiece {
        ChessPiece {
            value: 5,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: rook::rays().map(RayStatic::into_owned).to_vec(),
            capture_rays: None,
        }
    }

    pub fn queen() -> ChessPiece {
        ChessPiece {
            value: 9,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: queen::rays().to_vec(),
            capture_rays: None,
        }
    }

    pub fn king() -> ChessPiece {
        ChessPiece {
            value: usize::MAX,
            can_promote: false,
            valid_promotion: false,
            checkmate_possible: true,
            rays: king::rays().to_vec(),
            capture_rays: None,
        }
    }
}
