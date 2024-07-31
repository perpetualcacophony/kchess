#[derive(Debug, Copy, Clone)]
pub struct RayBuilder<Direction = crate::Direction> {
    pub(super) limit: Option<usize>,
    pub(super) direction: Direction,
}

impl<Direction> RayBuilder<Direction> {
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            limit: None,
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            direction: self.direction,
            limit: Some(limit),
        }
    }

    pub fn once(self) -> Self {
        self.limit(1)
    }
}
