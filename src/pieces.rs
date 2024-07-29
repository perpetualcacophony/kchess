use std::usize;

use crate::{direction::ray::Ray, ChessSide};

pub mod pawn;

pub mod knight;

pub mod bishop;

pub mod rook;

pub mod queen;

pub mod king;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPiece {
    pub const MINOR_PIECES: [Self; 2] = [Self::Knight, Self::Bishop];
    pub const MAJOR_PIECES: [Self; 2] = [Self::Rook, Self::Queen];
}

impl ChessPiece {
    pub const fn value(self) -> usize {
        match self {
            Self::Pawn => 1,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => usize::MAX,
        }
    }

    pub const fn can_promote(self) -> bool {
        matches!(self, Self::Pawn)
    }
}

pub enum PromotionOptions {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub struct ChessPieceStruct {
    value: usize,
    can_promote: bool,
    valid_promotion: bool,
    checkmate_possible: bool,
    rays: Vec<Box<dyn Ray>>,
    capture_rays: Option<Vec<Box<dyn Ray>>>,
}

impl ChessPieceStruct {
    pub fn pawn(side: ChessSide, moved: bool) -> Self {
        Self {
            value: 1,
            can_promote: true,
            valid_promotion: false,
            checkmate_possible: false,
            rays: vec![pawn::step_ray(side, moved).boxed()],
            capture_rays: Some(pawn::capture_rays(side).map(Ray::boxed).into()),
        }
    }

    pub fn knight() -> Self {
        Self {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: knight::rays().map(Ray::boxed).into(),
            capture_rays: None,
        }
    }

    pub fn bishop() -> Self {
        Self {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: bishop::rays().map(Ray::boxed).into(),
            capture_rays: None,
        }
    }

    pub fn rook() -> Self {
        Self {
            value: 5,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: rook::rays().map(Ray::boxed).into(),
            capture_rays: None,
        }
    }

    pub fn queen() -> Self {
        Self {
            value: 9,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: queen::rays().map(Ray::boxed).into(),
            capture_rays: None,
        }
    }

    pub fn king() -> Self {
        Self {
            value: usize::MAX,
            can_promote: false,
            valid_promotion: false,
            checkmate_possible: true,
            rays: king::rays().map(Ray::boxed).into(),
            capture_rays: None,
        }
    }
}
