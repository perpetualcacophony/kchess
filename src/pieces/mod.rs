use std::fmt::Debug;

use crate::direction::ray;

pub mod standard;
pub use standard::{Bishop, King, Knight, Pawn, Queen, Rook};

#[cfg(feature = "fairy")]
pub mod fairy;

mod primitive;
pub use primitive::PrimitivePiece;

mod stats;
pub use stats::Stats as PieceStats;

pub mod set;
pub use set::PieceSet as Set;

pub mod component;
pub use component::{PieceComponent as Component, PieceComponents as Components};

#[derive(Debug)]
pub struct ChessPiece {
    components: Components,
}

impl ChessPiece {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.components.get()
    }

    pub fn rays(&self) -> &ray::Set {
        self.get().unwrap()
    }

    pub fn stats(&self) -> &PieceStats {
        self.get().unwrap()
    }

    pub fn can_promote(&self) -> bool {
        self.stats().can_promote
    }

    pub fn valid_promotion(&self) -> bool {
        self.stats().valid_promotion
    }

    pub fn from_builder(builder: PieceBuilder) -> Option<Self> {
        builder.build()
    }

    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self::from_builder(PieceBuilder::new(P::build)).unwrap()
    }
}

#[derive(Default, Debug)]
pub struct PieceBuilder {
    components: component::Builder,
    rays: bool,
    stats: bool,
}

impl PieceBuilder {
    pub fn new(inner: impl FnOnce(&mut Self) -> &mut Self) -> Self {
        let mut new = Self::default();
        inner(&mut new);
        new
    }

    pub fn rays(&mut self, value: ray::Set) -> &mut Self {
        self.rays = true;
        self.add_component(Component::new(value));
        self
    }

    pub fn stats(&mut self, value: PieceStats) -> &mut Self {
        self.stats = true;
        self.add_component(Component::new(value));
        self
    }

    pub fn build(self) -> Option<ChessPiece> {
        if !self.rays || !self.stats {
            return None;
        }

        Some(ChessPiece {
            components: self.components.build(),
        })
    }
}

impl std::ops::Deref for PieceBuilder {
    type Target = component::Builder;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl std::ops::DerefMut for PieceBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}
