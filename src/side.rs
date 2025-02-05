use crate::{direction::Cardinal, Direction};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ChessSide {
    inner: usize,
}

impl ChessSide {
    const fn new(inner: usize) -> Self {
        Self { inner }
    }

    pub const fn after(&self) -> Self {
        Self::new(self.number() + 1)
    }

    pub const fn number(&self) -> usize {
        self.inner
    }

    pub const fn forward_cardinal(&self) -> Cardinal {
        Cardinal::ARRAY[self.number() % 3]
    }

    pub fn vec(count: usize) -> Vec<Self> {
        (0..count).map(Self::new).collect()
    }

    pub const fn two() -> [Self; 2] {
        [Self::new(0), Self::new(1)]
    }

    pub fn is_forward(&self, direction: &impl Direction) -> bool {
        direction.contains_cardinal(self.forward_cardinal())
    }

    #[cfg(feature = "rand")]
    pub fn choose_with<R: rand::Rng>(rng: &mut R, count: usize) -> Self {
        use rand::seq::SliceRandom;

        *Self::vec(count)
            .choose(rng)
            .expect("array should not be empty")
    }

    #[cfg(feature = "rand_full")]
    pub fn choose(count: usize) -> Self {
        Self::choose_with(&mut rand::thread_rng(), count)
    }
}
