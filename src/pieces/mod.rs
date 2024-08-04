use std::borrow::Borrow;

use crate::direction::ray;

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

    fn ray_enabled(_piece: &crate::components::Piece<'_>, _ray: &crate::direction::Ray) -> bool {
        true
    }
}

pub trait PieceSet {
    type Item;

    fn pieces() -> impl IntoIterator<Item = Self::Item>;

    fn data(&self) -> PieceData;

    fn promotions() -> Vec<Self>
    where
        Self: Sized;
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

impl PieceSet for Standard {
    type Item = StandardItem;

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

    fn pieces() -> impl IntoIterator<Item = Self::Item> {
        [
            StandardItem::Pawn(Pawn),
            StandardItem::Knight(Knight),
            StandardItem::Bishop(Bishop),
            StandardItem::Rook(Rook),
            StandardItem::Queen(Queen),
            StandardItem::King(King),
        ]
    }
}

pub enum StandardItem {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}
