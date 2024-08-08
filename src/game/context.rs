use crate::{board, pieces::PieceSet};

use super::{space::SpaceContext, AllPieces, Game};

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
