use crate::{pieces::PieceSet, Direction};

use super::Step;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RayBuilder {
    pub(super) limit: Option<usize>,
    pub(super) step: Step,
    pub(super) capture: bool,
    pub(super) predicate: fn(&dyn PieceSet) -> bool,
}

impl RayBuilder {
    pub fn new(direction: impl Direction) -> Self {
        Self {
            limit: None,
            step: direction.as_step(),
            capture: true,
            predicate: |_| true,
        }
    }

    pub fn some_limit(self, limit: usize) -> Self {
        self.limit(Some(limit))
    }

    pub fn limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }
    pub fn once(self) -> Self {
        self.some_limit(1)
    }

    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = capture;
        self
    }

    pub fn predicate(mut self, predicate: fn(&dyn PieceSet) -> bool) -> Self {
        self.predicate = predicate;
        self
    }
}
