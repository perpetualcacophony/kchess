use std::{borrow::Borrow, fmt::Debug, sync::Arc};

use crate::{direction::ray, game::piece::PartialPiece};

pub mod standard;
pub use standard::{Bishop, King, Knight, Pawn, Queen, Rook};

#[cfg(feature = "fairy")]
pub mod fairy;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceStats {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
}

impl PieceStats {
    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceData {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: ray::Set,
}

impl PieceData {
    pub fn from_primitive<P: PrimitivePiece>(piece: impl Borrow<P>) -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
            rays: ray::Set::new(|builder| piece.borrow().add_rays(builder)),
        }
    }
}

pub trait PrimitivePiece: Sized {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut ray::set::Builder) -> &'rays mut ray::set::Builder;

    fn ray_enabled(_piece: &PartialPiece, _ray: &crate::direction::Ray) -> bool {
        true
    }
}

pub trait Piece {
    fn stats(&self) -> PieceStats;

    fn rays(&self) -> ray::Set;
}

impl<T: PrimitivePiece> Piece for T {
    fn stats(&self) -> PieceStats {
        PieceStats::from_primitive::<Self>()
    }

    fn rays(&self) -> ray::Set {
        ray::Set::new(|builder| self.add_rays(builder))
    }
}

pub trait PieceSet {
    type Piece: Piece + Debug + PartialEq + Eq;

    fn pieces(&self) -> impl IntoIterator<Item = Arc<StandardPiece<Self::Piece>>>;

    fn promotions(&self) -> impl Iterator<Item = Arc<StandardPiece<Self::Piece>>> {
        self.pieces()
            .into_iter()
            .filter(|piece| piece.kind.stats().valid_promotion)
    }
}

#[derive(Debug)]
pub struct StandardSet {
    pieces: [Arc<StandardPiece<StandardPieceKind>>; 6],
}

impl PieceSet for StandardSet {
    type Piece = StandardPieceKind;

    fn pieces(&self) -> impl IntoIterator<Item = Arc<StandardPiece<Self::Piece>>> {
        self.pieces.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StandardPiece<Kind> {
    pub kind: Kind,
    pub stats: PieceStats,
    pub rays: ray::Set,
}

impl<Kind: Piece> StandardPiece<Kind> {
    pub fn new(kind: Kind) -> Self {
        Self {
            stats: kind.stats(),
            rays: kind.rays(),
            kind,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StandardPieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece for StandardPieceKind {
    fn stats(&self) -> PieceStats {
        match self {
            Self::Pawn => Pawn::stats(&Pawn),
            Self::Knight => Knight::stats(&Knight),
            Self::Bishop => Bishop::stats(&Bishop),
            Self::Rook => Rook::stats(&Rook),
            Self::Queen => Queen::stats(&Queen),
            Self::King => King::stats(&King),
        }
    }

    fn rays(&self) -> ray::Set {
        match self {
            Self::Pawn => Pawn.rays(),
            Self::Knight => Knight.rays(),
            Self::Bishop => Bishop.rays(),
            Self::Rook => Rook.rays(),
            Self::Queen => Queen.rays(),
            Self::King => King.rays(),
        }
    }
}

pub trait CustomData<Data>: PrimitivePiece {
    fn data(&self, core: PieceData) -> Data;
}

impl<T> CustomData<PieceData> for T
where
    T: PrimitivePiece,
{
    fn data(&self, core: PieceData) -> PieceData {
        core
    }
}
