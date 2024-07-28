use crate::{ChessPiece, ChessSide, Space};

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
    piece: Option<ChessPiece>,
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

type PiecesBase<'c> = std::collections::hash_map::Values<'c, crate::EntityId, Components>;

#[derive(Debug)]
pub struct Pieces<'c> {
    base: PiecesBase<'c>,
}

impl<'c> Pieces<'c> {
    pub fn get(game: &'c crate::Game) -> Self {
        Self {
            base: game.map.values(),
        }
    }
}

impl<'c> Iterator for Pieces<'c> {
    type Item = Piece<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().and_then(Piece::get)
    }
}
