use crate::{direction::Cardinal, ChessSide, Space};

mod dimensions;
pub use dimensions::BoardDimensions as Dimensions;

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub dimensions: Dimensions,
}

impl Board {
    pub fn spaces(&self) -> impl Iterator<Item = Space> + '_ {
        self.dimensions.spaces()
    }
}
