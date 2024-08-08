use super::PrimitivePiece;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Stats {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
}

impl Stats {
    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
        }
    }
}
