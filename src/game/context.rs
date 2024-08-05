use crate::board;

use super::{AllPieces, Game};

#[derive(Clone, Debug, Copy)]
pub struct GameContext<'a> {
    game: &'a Game,
}

impl<'a> GameContext<'a> {
    pub(super) fn from_game(game: &'a Game) -> Self {
        Self { game }
    }

    pub fn pieces(&self) -> AllPieces<'a> {
        self.game.pieces()
    }

    pub fn board(&self) -> &'a board::Dimensions {
        &self.game.board
    }
}
