use crate::Space;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoardDimensions {
    ranks: usize,
    files: usize,
}

impl BoardDimensions {
    pub fn new(ranks: usize, files: usize) -> Self {
        Self { ranks, files }
    }

    pub fn spaces(&self) -> impl Iterator<Item = Space> + '_ {
        (0..self.ranks).flat_map(|rank| (0..self.files).map(move |file| Space::new(rank, file)))
    }

    pub fn valid_space(&self, space: &Space) -> bool {
        self.spaces().any(|item| &item == space)
    }

    pub fn check_iter<'b>(
        &'b self,
        iter: impl IntoIterator<Item = Space> + 'b,
    ) -> impl Iterator<Item = Space> + 'b {
        iter.into_iter()
            .filter(|unchecked| self.valid_space(unchecked))
    }

    pub fn max_rank(&self) -> usize {
        self.ranks - 1
    }

    pub fn max_file(&self) -> usize {
        self.files - 1
    }

    /*     pub fn opposite_end(&self, side: ChessSide, space: Space) -> bool {
        let direction = side.forward_cardinal();

        (direction == Cardinal::NORTH && space.rank() == self.max_rank())
            || (direction == Cardinal::EAST && space.file() == self.max_file())
            || (direction == Cardinal::SOUTH && space.rank() == 0)
            || (direction == Cardinal::WEST && space.file() == 0)
    } */
}

impl Default for BoardDimensions {
    fn default() -> Self {
        Self::new(8, 8)
    }
}
