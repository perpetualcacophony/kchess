pub mod pawn;

pub mod knight;

pub mod bishop;

pub mod rook;

pub mod queen;

pub mod king;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChessPiece {
    Pawn { moved: bool },
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPiece {
    pub const fn moved(self) -> bool {
        match self {
            Self::Pawn { moved } => moved,
            _ => false,
        }
    }

    pub const fn value(self) -> usize {
        match self {
            Self::Pawn { .. } => 1,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => usize::MAX,
        }
    }
}
