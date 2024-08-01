use crate::direction::Cardinal;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
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

    #[cfg(feature = "rand")]
    pub fn choose_with<R: rand::Rng>(rng: &mut R) -> Self {
        use rand::seq::IteratorRandom;

        Self::ARRAY
            .into_iter()
            .choose(rng)
            .expect("array should not be empty")
    }

    #[cfg(feature = "rand_full")]
    pub fn choose() -> Self {
        Self::choose_with(&mut rand::thread_rng())
    }
}
