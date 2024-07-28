use crate::{components, Board, ChessSide, Components, EntityId, Space};

pub struct Game {
    board: Board,
    pub map: std::collections::HashMap<EntityId, Components>,
}

impl Game {
    pub fn components(&self) -> impl Iterator<Item = &Components> {
        self.map.values()
    }

    pub fn piece_on(&self, space: Space) -> Option<components::Piece<'_>> {
        self.pieces().find(|piece| piece.space == &space)
    }

    pub fn pieces(&self) -> components::Pieces<'_> {
        components::Pieces::get(self)
    }

    pub fn grid(&self) -> Vec<Vec<components::Piece<'_>>> {
        self.board
            .grid()
            .into_iter()
            .map(|rank| {
                rank.into_iter()
                    .filter_map(|space| self.piece_on(space))
                    .collect()
            })
            .collect()
    }

    pub fn sides(&self) -> [components::Side<'_>; 2] {
        [
            components::Side {
                side: ChessSide::White,
                pieces: self
                    .pieces()
                    .filter(|piece| piece.side == &ChessSide::White)
                    .collect(),
            },
            components::Side {
                side: ChessSide::Black,
                pieces: self
                    .pieces()
                    .filter(|piece| piece.side == &ChessSide::Black)
                    .collect(),
            },
        ]
    }
}
