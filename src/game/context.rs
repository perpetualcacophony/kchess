use crate::{board, pieces::PieceSet};

use super::{space::SpaceContext, AllPieces, Game};

#[derive(Clone, Debug, Copy)]
pub struct GameContext<'a, Set: PieceSet> {
    game: &'a Game<Set>,
}

impl<'a, Set: PieceSet> GameContext<'a, Set> {
    pub(super) fn from_game(game: &'a Game<Set>) -> Self {
        Self { game }
    }

    pub fn pieces(&self) -> AllPieces<'a, Set> {
        self.game.pieces()
    }

    pub fn board(&self) -> &'a board::BoardDimensions {
        &self.game.board
    }

    pub fn spaces(&self) -> impl Iterator<Item = SpaceContext<Set>> {
        self.game
            .board
            .spaces()
            .map(|space| SpaceContext::new(self, space))
    }
}
