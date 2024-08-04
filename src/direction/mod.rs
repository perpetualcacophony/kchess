pub mod cardinal;

pub use cardinal::Cardinal;

mod diagonal;
pub use diagonal::Diagonal;

pub mod ray;
pub use ray::Ray;

mod step;
pub use step::Step;

pub trait Direction {
    fn as_step(&self) -> step::Step;

    fn next_space(&self, start: crate::UncheckedSpace) -> Option<crate::UncheckedSpace> {
        self.as_step().next_space(start)
    }

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool;

    fn parse_step(step: step::Step) -> Option<Self>
    where
        Self: Sized;
}
