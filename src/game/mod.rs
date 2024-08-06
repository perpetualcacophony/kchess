use crate::{board, ChessSide, Space};

mod space;

pub mod pieces;

mod context;
pub type Context<'a> = &'a context::GameContext<'a>;

pub mod side;
pub use side::Side;

pub mod piece;
pub use piece::Piece;

pub type AllPieces<'a> = pieces::AllPieces<std::slice::Iter<'a, Piece>>;

#[derive(Clone, Debug)]
pub struct Game {
    board: board::BoardDimensions,
    pieces: Vec<Piece>,
}

impl Game {
    pub fn piece_on(&self, space: Space) -> Option<&Piece> {
        self.pieces().find(|piece| piece.space == space)
    }

    pub fn pieces(&self) -> AllPieces {
        AllPieces::new(&self.pieces)
    }

    pub fn sides(&self) -> [Side<'_>; 2] {
        ChessSide::two().map(|side| Side {
            side,
            pieces: self.pieces().filter(|piece| piece.side == side).collect(),
        })
    }

    pub fn context(&self) -> context::GameContext<'_> {
        context::GameContext::from_game(self)
    }
}
