use std::borrow::Borrow;

use crate::direction::ray::{RaySet, RaySetBuilder};

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
    pub rays: RaySet,
}

impl PieceData {
    pub fn from_primitive<P: PrimitivePiece>(piece: impl Borrow<P>) -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
            rays: RaySet::from_builder(|builder| {
                piece.borrow().add_rays(builder);
            }),
        }
    }
}

pub trait PrimitivePiece: Sized {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder;
}

pub trait PieceSet {
    fn data(&self) -> PieceData;

    fn promotions() -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece<Set> {
    pub inner: Set,
    pub data: PieceData,
}

impl<Set> Piece<Set>
where
    Set: PieceSet,
{
    pub fn new(inner: impl Into<Set>) -> Self {
        let inner = inner.into();

        Self {
            data: inner.data(),
            inner,
        }
    }
}

impl<Set> Piece<Set> {
    pub fn rays(&self) -> &RaySet {
        &self.data.rays
    }
}

macro_rules! piece_set {
    ($name:ident: $($primitive:ident),*) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $primitive($primitive)
            ),*
        }

        impl $name {
            fn piece_data(&self) -> PieceData {
                match self {
                    $(
                        Self::$primitive(inner) => PieceData::from_primitive::<$primitive>(inner)
                    ),*
                }
            }
        }

        $(
            impl From<$primitive> for $name {
                fn from(value: $primitive) -> Self {
                    Self::$primitive(value)
                }
            }
        )*
    };
}

piece_set! {
    Standard:
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub type StandardPiece = Piece<Standard>;

impl PieceSet for Standard {
    fn data(&self) -> PieceData {
        self.piece_data()
    }

    fn promotions() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Knight(Knight),
            Self::Bishop(Bishop),
            Self::Rook(Rook),
            Self::Queen(Queen),
        ]
    }
}