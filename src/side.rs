#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChessSide {
    White,
    Black,
}

impl ChessSide {
    pub const ARRAY: [Self; 2] = [Self::White, Self::Black];

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
