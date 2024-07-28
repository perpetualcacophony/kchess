use crate::{pieces, Board, ChessPiece, ChessSide, Space};

#[derive(Clone, Debug, Default)]
pub struct Components {
    space: Option<Space>,
    piece: Option<ChessPiece>,
    side: Option<ChessSide>,
    captured: Option<bool>,
    moved: Option<bool>,
}

impl<'c> Piece<'c> {
    pub fn legal_moves(
        self,
        board: &Board,
        pieces: impl IntoIterator<Item = Self> + Copy,
    ) -> Vec<Space> {
        let mut moves = Vec::new();

        match self.piece {
            ChessPiece::Pawn => {
                let mut steps =
                    board.check_iter(pieces::pawn::steps(*self.side, self.space.as_unchecked()));

                if *self.moved {
                    if let Some(space) = steps.next() {
                        if !pieces.into_iter().any(|piece| piece.space == &space) {
                            moves.push(space)
                        }
                    }
                } else {
                    for space in steps {
                        if !pieces.into_iter().any(|piece| piece.space == &space) {
                            moves.push(space)
                        }
                    }
                }

                for space in board.check_iter(pieces::pawn::captures(
                    *self.side,
                    self.space.as_unchecked(),
                )) {
                    if let Some(piece) = pieces.into_iter().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        } else {
                            break;
                        }
                    } else {
                        moves.push(space)
                    }
                }
            }
            ChessPiece::Knight => {
                for space in board.check_iter(pieces::knight::moves(self.space.as_unchecked())) {
                    if let Some(piece) = pieces.into_iter().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        }
                    } else {
                        moves.push(space)
                    }
                }
            }

            ChessPiece::Bishop | ChessPiece::Rook | ChessPiece::Queen => {
                let rays: Vec<Box<dyn Iterator<Item = Space> + '_>> = match self.piece {
                    ChessPiece::Bishop => pieces::bishop::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    ChessPiece::Rook => pieces::rook::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    ChessPiece::Queen => pieces::queen::rays(self.space.as_unchecked())
                        .map(|ray| board.check_iter(ray))
                        .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = Space>>)
                        .into(),
                    _ => unreachable!(),
                };

                for mut ray in rays {
                    loop {
                        if let Some(space) = ray.next() {
                            if let Some(piece) =
                                pieces.into_iter().find(|piece| piece.space == &space)
                            {
                                if piece.side != self.side {
                                    moves.push(space)
                                } else {
                                    break;
                                }
                            } else {
                                moves.push(space)
                            }
                        }
                    }
                }
            }
            ChessPiece::King => {
                for space in board.check_iter(pieces::king::moves(self.space.as_unchecked())) {
                    if let Some(piece) = pieces.into_iter().find(|piece| piece.space == &space) {
                        if piece.side != self.side {
                            moves.push(space)
                        } else {
                            break;
                        }
                    } else {
                        moves.push(space)
                    }
                }

                todo!()
            }
        }

        moves
    }
}

impl<'c> PieceMut<'c> {
    pub fn capture(self) {}

    pub fn promote(self) {
        *self.piece = ChessPiece::Queen;
    }

    pub fn move_to(self, space: Space, board: &Board, mut pieces: impl Iterator<Item = Self>) {
        *self.moved = true;

        if let Some(piece) = pieces.find(|piece| *piece.space == space) {
            piece.capture()
        }

        *self.space = space;

        if *self.piece == ChessPiece::Pawn && board.last_rank(*self.side, self.space.rank()) {
            self.promote();
        }
    }
}

pub struct Side<'c> {
    pub side: ChessSide,
    pub pieces: Vec<Piece<'c>>,
}

impl<'c> Side<'c> {
    pub fn material(&self) -> usize {
        self.active_pieces()
            .fold(0, |total, piece| total + piece.piece.value())
    }

    pub fn active_pieces(&self) -> impl Iterator<Item = &Piece<'c>> {
        self.pieces.iter().filter(|piece| !piece.captured)
    }

    pub fn advantage(&self, rhs: &Self) -> isize {
        self.material() as isize - rhs.material() as isize
    }

    pub fn pieces(&self, kind: ChessPiece) -> impl Iterator<Item = &Piece<'c>> {
        self.active_pieces()
            .filter(move |piece| piece.piece == &kind)
    }
}

macro_rules! bundle {
    ($ident:ident $($field:ident: $type:ty),*) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $ident<'c> {
            $(
                pub $field: &'c $type
            ),*
        }

        impl<'c> $ident<'c> {
            pub fn get(components: &'c Components) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_ref()?
                    ),*
                })
            }
        }
    };

    (mut $ident:ident $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $ident<'c> {
            $(
                pub $field: &'c mut $type
            ),*
        }

        impl<'c> $ident<'c> {
            pub fn get(components: &'c mut Components) -> Option<Self> {
                Some( Self {
                    $(
                        $field: components.$field.as_mut()?
                    ),*
                })
            }
        }
    };
}

bundle! {
    Piece
    moved: bool,
    space: Space,
    piece: ChessPiece,
    side: ChessSide,
    captured: bool
}

bundle! {
    mut PieceMut
    moved: bool,
    space: Space,
    piece: ChessPiece,
    side: ChessSide,
    captured: bool
}

pub struct MoveTo<'c> {
    space: Space,
    capture: Option<&'c Piece<'c>>,
}

impl<'c> MoveTo<'c> {
    pub fn step(space: Space) -> Self {
        Self {
            space,
            capture: None,
        }
    }

    pub fn capture(space: Space, piece: &'c Piece<'c>) -> Self {
        Self {
            space,
            capture: Some(piece),
        }
    }
}
