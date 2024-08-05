use crate::Board;

use super::{AllPieces, Game};

pub struct GameContext<'a> {
    pub board: &'a Board,
    pub pieces: AllPieces<'a>,
}

impl<'a> GameContext<'a> {
    pub(super) fn from_game(game: &'a Game) -> Self {
        Self {
            board: &game.board,
            pieces: game.pieces(),
        }
    }
}
