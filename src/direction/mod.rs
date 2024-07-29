pub mod cardinal;
pub use cardinal::Cardinal;

pub mod diagonal;
pub use diagonal::Diagonal;

pub mod relative;
pub use relative::Relative;

pub mod ray;
pub use ray::InfiniteRay;

pub trait Direction: Copy {
    fn opposite(self) -> Self;
    fn next_space(self, start: crate::UncheckedSpace) -> crate::UncheckedSpace;
    fn perpendicular(self) -> [Self; 2];
}
