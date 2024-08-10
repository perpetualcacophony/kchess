use crate::{direction::ray, game::piece::PartialPiece};

use super::ChessPiece;

pub trait PrimitivePiece {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays(set: &mut ray::set::Builder) -> &mut ray::set::Builder;

    fn ray_enabled(_piece: &PartialPiece, _ray: &crate::direction::Ray) -> bool {
        true
    }

    fn chess_piece() -> ChessPiece
    where
        Self: Sized,
    {
        ChessPiece::from_primitive::<Self>()
    }

    fn build(builder: &mut super::PieceBuilder) -> &mut super::PieceBuilder
    where
        Self: Sized,
    {
        builder
            .rays(ray::Set::new(Self::add_rays))
            .stats(super::PieceStats::from_primitive::<Self>())
    }
}
