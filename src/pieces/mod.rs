use std::{any::Any, borrow::Borrow, collections::HashSet, fmt::Debug, sync::Arc};

use crate::{direction::ray, game::piece::PartialPiece};

pub mod standard;
pub use standard::{Bishop, King, Knight, Pawn, Queen, Rook};

macro_rules! piece_set {
    (
        $Name:ident:
        $(
            $Piece:ident$(: $path:path)?
        ),+
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $Name {
            $(
                $Piece
            ),+
        }

        impl $crate::pieces::PieceSet for $Name {
            fn stats(&self) -> $crate::pieces::PieceStats {
                use $crate::pieces::Piece;

                $(
                    use $($path::)?$Piece;
                )+

                match self {
                    $(
                        Self::$Piece => <$(<$path>::)?$Piece>::stats(&$(<$path>::)?$Piece)
                    ),+
                }
            }

            fn rays(&self) -> $crate::direction::ray::Set {
                use $crate::pieces::Piece;

                match self {
                    $(
                        Self::$Piece => <$(<$path>::)?$Piece>::rays(&$(<$path>::)?$Piece)
                    ),+
                }
            }
        }
    };
}

piece_set! {
    StandardSet:
    Pawn, Bishop, Knight, Rook, Queen, King
}

#[cfg(feature = "fairy")]
pub mod fairy;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceStats {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
}

impl PieceStats {
    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceData {
    pub value: usize,
    pub can_promote: bool,
    pub valid_promotion: bool,
    pub checkmate_possible: bool,
    pub rays: ray::Set,
}

impl PieceData {
    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self {
            value: P::VALUE,
            can_promote: P::CAN_PROMOTE,
            valid_promotion: P::VALID_PROMOTION,
            checkmate_possible: P::CHECKMATE_POSSIBLE,
            rays: ray::Set::new(|builder| P::add_rays(builder)),
        }
    }
}

pub trait PrimitivePiece {
    const VALUE: usize;
    const CAN_PROMOTE: bool = false;
    const VALID_PROMOTION: bool = true;
    const CHECKMATE_POSSIBLE: bool = false;

    fn add_rays(set: &mut ray::set::Builder) -> &mut ray::set::Builder;

    fn ray_enabled(_piece: &PartialPiece, _ray: &crate::direction::Ray) -> bool {
        true
    }
}

pub trait Piece {
    fn stats(&self) -> PieceStats;
    fn rays(&self) -> ray::Set;
}

impl<T: PrimitivePiece> Piece for T {
    fn stats(&self) -> PieceStats {
        PieceStats::from_primitive::<Self>()
    }

    fn rays(&self) -> ray::Set {
        todo!()
        //ray::Set::new(|builder| self.add_rays(builder))
    }
}

#[derive(Debug)]
pub struct ArcPiece<Set> {
    inner: Arc<ArcPieceInner<Set>>,
}

impl<T> Clone for ArcPiece<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArcPieceInner<Set> {
    pub piece: Set,
    pub stats: PieceStats,
    pub rays: ray::Set,
}

#[derive(Debug)]
pub struct PieceSetContainer<Set> {
    vec: Vec<ArcPiece<Set>>,
}

impl<Set> PieceSetContainer<Set> {
    fn iter(&self) -> impl Iterator<Item = ArcPiece<Set>> + '_ {
        self.vec.iter().cloned()
    }
}

pub trait PieceSet {
    fn stats(&self) -> PieceStats;
    fn rays(&self) -> ray::Set;
}

#[derive(Debug)]
pub struct PieceNew {
    components: PieceComponents,
}

impl PieceNew {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.components.get()
    }

    pub fn rays(&self) -> &ray::Set {
        self.get().unwrap()
    }

    pub fn stats(&self) -> &PieceStats {
        self.get().unwrap()
    }

    pub fn from_builder(builder: PieceBuilder) -> Option<Self> {
        builder.build()
    }

    pub fn from_primitive<P: PrimitivePiece>() -> Self {
        Self::from_builder(PieceBuilder::new(|builder| {
            builder
                .rays(ray::Set::new(P::add_rays))
                .stats(PieceStats::from_primitive::<P>())
        }))
        .unwrap()
    }
}

#[derive(Debug)]
pub struct PieceComponent {
    inner: Box<dyn Any>,
}

impl PieceComponent {
    pub fn new<T: 'static + Any>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }
}

#[derive(Debug)]
pub struct PieceComponents {
    inner: Vec<PieceComponent>,
}

impl PieceComponents {
    pub fn iter(&self) -> impl Iterator<Item = &PieceComponent> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PieceComponent> {
        self.inner.iter_mut()
    }
}

impl PieceComponents {
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.iter().find_map(PieceComponent::get)
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.get::<T>().is_some()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.iter_mut().find_map(PieceComponent::get_mut)
    }

    pub fn expect<T: 'static>(&self) -> &T {
        self.get().unwrap()
    }

    pub fn expect_mut<T: 'static>(&mut self) -> &mut T {
        self.get_mut().unwrap()
    }
}

impl FromIterator<PieceComponent> for PieceComponents {
    fn from_iter<T: IntoIterator<Item = PieceComponent>>(iter: T) -> Self {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}

#[derive(Default, Debug)]
pub struct PieceBuilder {
    components: PieceComponentsBuilder,
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
        self.add_component(PieceComponent::new(value));
        self
    }

    pub fn stats(&mut self, value: PieceStats) -> &mut Self {
        self.stats = true;
        self.add_component(PieceComponent::new(value));
        self
    }

    pub fn build(self) -> Option<PieceNew> {
        if !self.rays || !self.stats {
            return None;
        }

        Some(PieceNew {
            components: self.components.build(),
        })
    }
}

impl std::ops::Deref for PieceBuilder {
    type Target = PieceComponentsBuilder;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl std::ops::DerefMut for PieceBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}

#[derive(Default, Debug)]
pub struct PieceComponentsBuilder {
    inner: Vec<PieceComponent>,
}

impl PieceComponentsBuilder {
    pub fn build(self) -> PieceComponents {
        PieceComponents { inner: self.inner }
    }

    pub fn new(inner: impl FnOnce(&mut Self) -> &mut Self) -> Self {
        let mut new = Self::default();
        inner(&mut new);
        new
    }

    pub fn add_component(&mut self, component: PieceComponent) -> &mut Self {
        self.inner.push(component);
        self
    }

    pub fn add_components(
        &mut self,
        components: impl IntoIterator<Item = PieceComponent>,
    ) -> &mut Self {
        self.inner.extend(components);
        self
    }
}

impl Extend<PieceComponent> for PieceComponentsBuilder {
    fn extend<T: IntoIterator<Item = PieceComponent>>(&mut self, iter: T) {
        self.add_components(iter);
    }
}
