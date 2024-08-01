use crate::Direction;

use super::Step;

#[derive(Debug, Copy, Clone)]
pub struct RayBuilder {
    pub(super) limit: Option<usize>,
    pub(super) step: Step,
}

impl RayBuilder {
    pub fn new(direction: impl Direction) -> Self {
        Self {
            limit: None,
            step: direction.as_step(),
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit: Some(limit),
            step: self.step,
        }
    }

    pub fn once(self) -> Self {
        self.limit(1)
    }
}
