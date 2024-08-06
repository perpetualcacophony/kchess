use crate::{board, pieces::PieceSet, ChessSide, Space};

mod space;

pub mod pieces;

mod context;
pub type Context<'a, Set> = &'a context::GameContext<'a, Set>;

pub mod side;
pub use side::Side;

pub mod piece;
pub use piece::Piece;

pub type AllPieces<'a, Set> = pieces::AllPieces<std::slice::Iter<'a, Piece<Set>>>;

#[derive(Debug)]
pub struct Game<Set: PieceSet> {
    board: board::BoardDimensions,
    pieces: Vec<Piece<Set>>,
    piece_set: Set,
}

impl<Set: PieceSet> Game<Set> {
    pub fn piece_on(&self, space: Space) -> Option<&Piece<Set>> {
        self.pieces().find(|piece| piece.space() == &space)
    }

    pub fn pieces(&self) -> AllPieces<Set> {
        AllPieces::new(&self.pieces)
    }

    pub fn sides(&self) -> [Side<Set>; 2] {
        ChessSide::two().map(|side| Side {
            side,
            pieces: self
                .pieces()
                .filter(|piece| piece.side() == &side)
                .collect(),
        })
    }

    pub fn context(&self) -> context::GameContext<'_, Set> {
        context::GameContext::from_game(self)
    }
}
