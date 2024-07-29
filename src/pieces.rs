use std::usize;

pub mod pawn;

pub mod knight;

pub mod bishop;

pub mod rook;

pub mod queen;

pub mod king;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPiece {
    pub const MINOR_PIECES: [Self; 2] = [Self::Knight, Self::Bishop];
    pub const MAJOR_PIECES: [Self; 2] = [Self::Rook, Self::Queen];
}

impl ChessPiece {
    pub const fn value(self) -> usize {
        match self {
            Self::Pawn => 1,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => usize::MAX,
        }
    }

    pub const fn can_promote(self) -> bool {
        matches!(self, Self::Pawn)
    }
}

pub enum PromotionOptions {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub struct ChessPieceStruct {
    value: usize,
    can_promote: bool,
    valid_promotion: bool,
    checkmate_possible: bool,
}

impl ChessPieceStruct {
    pub const PAWN: Self = Self {
        value: 1,
        can_promote: true,
        valid_promotion: false,
        checkmate_possible: false,
    };

    pub const KNIGHT: Self = Self {
        value: 3,
        can_promote: false,
        valid_promotion: true,
        checkmate_possible: false,
    };

    pub const BISHOP: Self = Self {
        value: 3,
        can_promote: false,
        valid_promotion: true,
        checkmate_possible: false,
    };

    pub const ROOK: Self = Self {
        value: 5,
        can_promote: false,
        valid_promotion: true,
        checkmate_possible: false,
    };

    pub const QUEEN: Self = Self {
        value: 9,
        can_promote: false,
        valid_promotion: true,
        checkmate_possible: false,
    };

    pub const KING: Self = Self {
        value: usize::MAX,
        can_promote: false,
        valid_promotion: false,
        checkmate_possible: true,
    };
}
