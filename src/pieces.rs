use crate::{
    direction::ray::{Ray, RayStatic},
    ChessSide,
};

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

#[derive(Clone, Debug)]
pub struct ChessPieceStruct {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: Vec<Ray>,
    pub capture_rays: Option<Vec<Ray>>,
}

impl ChessPieceStruct {
    pub fn pawn(side: ChessSide, moved: bool) -> ChessPieceStruct {
        ChessPieceStruct {
            value: 1,
            can_promote: true,
            valid_promotion: false,
            checkmate_possible: false,
            rays: vec![pawn::step_ray(side, moved).into_vec()],
            capture_rays: Some(pawn::capture_rays(side).map(|ray| ray.into_vec()).to_vec()),
        }
    }

    pub fn knight() -> ChessPieceStruct {
        ChessPieceStruct {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: knight::rays().map(|ray| ray.into_vec()).to_vec(),
            capture_rays: None,
        }
    }

    pub fn bishop() -> ChessPieceStruct {
        ChessPieceStruct {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: bishop::rays().map(RayStatic::into_vec).to_vec(),
            capture_rays: None,
        }
    }

    pub fn rook() -> ChessPieceStruct {
        ChessPieceStruct {
            value: 5,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: rook::rays().map(RayStatic::into_vec).to_vec(),
            capture_rays: None,
        }
    }

    pub fn queen() -> ChessPieceStruct {
        ChessPieceStruct {
            value: 9,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: queen::rays().map(|ray| ray.into_vec()).to_vec(),
            capture_rays: None,
        }
    }

    pub fn king() -> ChessPieceStruct {
        ChessPieceStruct {
            value: usize::MAX,
            can_promote: false,
            valid_promotion: false,
            checkmate_possible: true,
            rays: king::rays().map(|ray| ray.into_vec()).to_vec(),
            capture_rays: None,
        }
    }
}

/* pub trait PieceTrait {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;
}

pub struct Pawn;

pub struct Knight;

impl PieceTrait for Knight {
    const VALUE: usize = 3;

    fn rays(&self) -> impl Iterator<Item = Self::Ray> {
        knight::rays().into_iter()
    }
}

pub struct Bishop;

impl PieceTrait for Bishop {
    type Ray = bishop::BishopRay;

    const VALUE: usize = 3;

    fn rays(&self) -> impl Iterator<Item = Self::Ray> {
        bishop::rays().into_iter()
    }
}

pub enum PieceSet {
    Knight(Knight),
    Bishop(Bishop),
}

impl PieceSet {
    fn cast(
        &self,
        start: UncheckedSpace,
    ) -> impl Iterator<Item = impl Iterator<Item = UncheckedSpace>> {
        match self {
            Self::Knight(knight) => knight.rays().map(|ray| ray.cast(start)),
            Self::Bishop(bishop) => bishop.rays().map(|ray| ray.cast(start)),
        }
    }
}
 */
