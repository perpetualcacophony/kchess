use crate::board;

use super::{space::SpaceContext, AllPieces, Game};

#[derive(Clone, Debug, Copy)]
pub struct GameContext<'a, 'set, Set> {
    game: &'a Game<'set, Set>,
}

impl<'a, 'set, Set> GameContext<'a, 'set, Set> {
    pub(super) fn from_game(game: &'a Game<'set, Set>) -> Self {
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
