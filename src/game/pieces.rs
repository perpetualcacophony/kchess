use crate::{components::Piece, Components, EntityId};

use super::Game;

pub struct Pieces<'c> {
    inner: std::collections::hash_map::Values<'c, EntityId, Components>,
}

impl<'c> Pieces<'c> {
    pub(super) fn get(game: &'c Game) -> Self {
        Self {
            inner: game.map.values(),
        }
    }
}

impl<'c> Iterator for Pieces<'c> {
    type Item = Piece<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(Piece::get)
    }
}
