use crate::{
    pieces::{Pawn, PieceData, Standard},
    ChessSide, Space,
};

#[macro_use]
mod macros;

pub mod piece;
pub use piece::Piece;

pub mod piece_mut;
pub use piece_mut::PieceMut;

pub mod side;
pub use side::Side;

#[derive(Clone, Debug, Default)]
pub struct Components {
    space: Option<Space>,
    piece: Option<PieceData>,
    side: Option<ChessSide>,
    captured: Option<bool>,
    moved: Option<bool>,
}

/* pub struct MoveTo<'c> {
    space: Space,
    capture: Option<&'c Piece<'c>>,
}

impl<'c> MoveTo<'c> {
    pub fn step(space: Space) -> Self {
        Self {
            space,
            capture: None,
        }
    }

    pub fn capture(space: Space, piece: &'c Piece<'c>) -> Self {
        Self {
            space,
            capture: Some(piece),
        }
    }
} */
