use crate::{ChessSide, Space, UncheckedSpace};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub ranks: usize,
    pub files: usize,
}

impl Board {
    pub fn space(&self, rank: usize, file: usize) -> Option<Space> {
        if rank < self.ranks && file < self.ranks {
            Some(unsafe { Space::new_unchecked(rank, file) })
        } else {
            None
        }
    }

    pub fn expect_space(&self, rank: usize, file: usize) -> Space {
        assert!(rank < self.ranks);
        assert!(file < self.files);

        self.space(rank, file).unwrap()
    }

    pub fn check_space(&self, unchecked: UncheckedSpace) -> Option<Space> {
        self.space(unchecked.rank, unchecked.file)
    }

    pub fn check_iter<'b>(
        &'b self,
        iter: impl IntoIterator<Item = UncheckedSpace> + 'b,
    ) -> impl Iterator<Item = Space> + 'b {
        iter.into_iter()
            .filter_map(|unchecked| self.check_space(unchecked))
    }

    pub fn last_rank(&self, side: ChessSide, rank: usize) -> bool {
        (side == ChessSide::White && rank == self.ranks - 1)
            || (side == ChessSide::Black && rank == 0)
    }

    pub fn grid(&self) -> Vec<Vec<Space>> {
        let mut ranks = Vec::with_capacity(self.ranks);

        for rank in (0..self.ranks).rev() {
            let mut spaces = Vec::with_capacity(self.files);

            for file in 0..self.files {
                spaces.push(self.expect_space(rank, file))
            }

            ranks.push(spaces)
        }

        ranks
    }
}
