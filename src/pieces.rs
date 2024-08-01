use std::borrow::Borrow;

use crate::direction::ray::{RaySet, RaySetBuilder};

mod pawn;
pub use pawn::Pawn;

mod knight;
pub use knight::Knight;

mod bishop;
pub use bishop::Bishop;

mod rook;
pub use rook::Rook;

mod queen;
pub use queen::Queen;

mod king;
pub use king::King;

#[derive(Clone, Debug, PartialEq)]
pub struct PieceData {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: RaySet,
}

impl PieceData {
    pub fn from_kind<P: PieceKind>(piece: impl Borrow<P>) -> Self {
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

pub trait PieceKind: Sized {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder;
}
