use crate::{board, pieces::Set, ChessSide, Space};

mod space;

pub mod pieces;

mod context;
pub type Context<'a, 'set, Set> = &'a context::GameContext<'a, 'set, Set>;

pub mod side;
pub use side::Side;

pub mod piece;
pub use piece::Piece;

pub type AllPieces<'a, 'set> = pieces::AllPieces<std::slice::Iter<'a, Piece<'set>>>;

#[derive(Debug)]
pub struct Game<'set, Set> {
    board: board::BoardDimensions,
    pieces: Vec<Piece<'set>>,
    piece_set: Set,
}

impl<'set, Set> Game<'set, Set> {
    pub fn piece_on(&self, space: Space) -> Option<&Piece<'set>> {
        self.pieces().find(|piece| piece.space() == &space)
    }

    pub fn pieces(&self) -> AllPieces<'_, 'set> {
        AllPieces::new(&self.pieces)
    }

    pub fn sides(&self) -> [Side; 2] {
        ChessSide::two().map(|side| Side {
            side,
            pieces: self
                .pieces()
                .filter(|piece| piece.side() == &side)
                .collect(),
        })
    }

    pub fn context(&self) -> context::GameContext<'_, 'set, Set> {
        context::GameContext::from_game(self)
    }
}
