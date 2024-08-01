use crate::Direction;

use super::Step;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RayBuilder {
    pub(super) limit: Option<usize>,
    pub(super) step: Step,
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

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn once(self) -> Self {
        self.limit(1)
    }

    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = capture;
        self
    }
}
