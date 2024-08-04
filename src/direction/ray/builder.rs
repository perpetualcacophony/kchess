use crate::Direction;

use super::modname::Step;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RayBuilder {
    pub(super) limit: Option<usize>,
    pub(super) step: modname::Step,
    pub(super) capture: bool,
}

impl RayBuilder {
    pub fn new(direction: impl Direction) -> Self {
        Self {
            limit: None,
            step: direction.as_step(),
            capture: true,
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
}
