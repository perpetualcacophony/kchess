use crate::{components::Piece, Components, EntityId};

use super::Game;

#[derive(Clone)]
pub struct AllPieces<'c> {
    inner: std::collections::hash_map::Values<'c, EntityId, Components>,
}

impl<'c> AllPieces<'c> {
    pub(super) fn get(game: &'c Game) -> Self {
        Self {
            inner: game.map.values(),
        }
    }
}

impl<'c> Iterator for AllPieces<'c> {
    type Item = Piece<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(Piece::get)
    }
}

impl<'c> AllPieces<'c> {
    pub fn not_captured(self) -> impl Iterator<Item = Piece<'c>> {
        self.filter(|piece| !piece.captured)
    }
}
