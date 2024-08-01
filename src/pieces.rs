use crate::{
    direction::{
        ray::{Ray, RayBuilder, RaySet},
        Cardinal, Diagonal,
    },
    ChessSide,
};

pub mod pawn;

pub mod knight;

pub mod bishop;

pub mod rook;

pub mod queen;

pub mod king;

#[derive(Clone, Debug, PartialEq)]
pub struct PieceData {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: RaySet,
    pub capture_rays: Option<RaySet>,
}

impl PieceData {
    pub fn pawn(side: ChessSide, moved: bool) -> PieceData {
        PieceData {
            value: 1,
            can_promote: true,
            valid_promotion: false,
            checkmate_possible: false,
            rays: pawn::step_ray(side, moved),
            capture_rays: Some(pawn::capture_rays(side)),
        }
    }

    pub fn knight() -> PieceData {
        PieceData {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: knight::rays(),
            capture_rays: None,
        }
    }

    pub fn bishop() -> PieceData {
        PieceData {
            value: 3,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: bishop::rays(),
            capture_rays: None,
        }
    }

    pub fn rook() -> PieceData {
        PieceData {
            value: 5,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: rook::rays(),
            capture_rays: None,
        }
    }

    pub fn queen() -> PieceData {
        PieceData {
            value: 9,
            can_promote: false,
            valid_promotion: true,
            checkmate_possible: false,
            rays: queen::rays(),
            capture_rays: None,
        }
    }

    pub fn king() -> PieceData {
        PieceData {
            value: usize::MAX,
            can_promote: false,
            valid_promotion: false,
            checkmate_possible: true,
            rays: king::rays(),
            capture_rays: None,
        }
    }

    pub fn from_kind<P: PieceKind>() -> Self {
        let mut rays = RaySet::new();
        P::add_rays(&mut rays);

        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
            rays,
            capture_rays: None,
        }
    }
}

pub trait PieceKind: Sized {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays(set: &mut RaySet) -> &mut RaySet;
}

pub struct Bishop;

impl PieceKind for Bishop {
    const VALUE: usize = 3;

    fn add_rays(set: &mut RaySet) -> &mut RaySet {
        set.add_many(Diagonal::ARRAY.map(RayBuilder::new))
    }
}

pub struct Rook;

impl PieceKind for Rook {
    const VALUE: usize = 5;

    fn add_rays(set: &mut RaySet) -> &mut RaySet {
        set.add_many(Cardinal::ARRAY.map(RayBuilder::new))
    }
}

pub struct Queen;

impl PieceKind for Queen {
    const VALUE: usize = 9;

    fn add_rays(set: &mut RaySet) -> &mut RaySet {
        set.add_piece::<Bishop>().add_piece::<Rook>()
    }
}
