use crate::board;

use super::{space::SpaceContext, AllPieces, Game};

#[derive(Clone, Debug, Copy)]
pub struct GameContext<'a, 'set> {
    game: &'a Game<'set>,
}

impl<'a, 'set> GameContext<'a, 'set> {
    pub(super) fn from_game(game: &'a Game<'set>) -> Self {
        Self { game }
    }

    pub fn pieces(&self) -> AllPieces<'a, 'set> {
        self.game.pieces()
    }

    pub fn board(&self) -> &'a board::BoardDimensions {
        &self.game.board
    }

    pub fn spaces(&self) -> impl Iterator<Item = SpaceContext> {
        self.game
            .board
            .spaces()
            .map(|space| SpaceContext::new(self, space))
    }
}
