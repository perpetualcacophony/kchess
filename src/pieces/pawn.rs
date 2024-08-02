use crate::{
    direction::{
        ray::{RayBuilder, RaySetBuilder},
        Cardinal, Diagonal,
    },
    ChessSide, Direction,
};

use super::PrimitivePiece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pawn {
    moved: bool,
    side: ChessSide,
}

impl PrimitivePiece for Pawn {
    const VALUE: usize = 1;
    const CAN_PROMOTE: bool = true;
    const VALID_PROMOTION: bool = false;

    fn add_rays<'rays>(&self, set: &'rays mut RaySetBuilder) -> &'rays mut RaySetBuilder {
        let limit = if self.moved { 1 } else { 2 };

        set.add(RayBuilder::new(Diagonal::NORTHEAST.relative(self.side)).once())
            .add(RayBuilder::new(Diagonal::NORTHEAST.relative(self.side)).once())
            .add(
                RayBuilder::new(Cardinal::NORTH.relative(self.side))
                    .some_limit(limit)
                    .capture(false),
            )
    }
}
