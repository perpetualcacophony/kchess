use crate::{board, ChessSide, EntityId, Space};

mod space;

pub mod pieces;
use components::Piece;

mod context;
pub use context::GameContext as Context;

pub mod components;

pub type AllPieces<'a> = pieces::AllPieces<std::slice::Iter<'a, Piece>>;

#[derive(Clone, Debug)]
pub struct Game {
    board: board::Dimensions,
    pieces: Vec<components::Piece>,
}

impl Game {
    pub fn piece_on(&self, space: Space) -> Option<&components::Piece> {
        self.pieces().find(|piece| piece.space == space)
    }

    pub fn pieces(&self) -> AllPieces {
        AllPieces::new(&self.pieces)
    }

    pub fn sides(&self) -> [components::Side<'_>; 2] {
        ChessSide::two().map(|side| components::Side {
            side,
            pieces: self.pieces().filter(|piece| piece.side == side).collect(),
        })
    }

    pub fn context(&self) -> Context<'_> {
        Context::from_game(self)
    }
}
