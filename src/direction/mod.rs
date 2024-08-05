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

    fn next_space(&self, start: &crate::Space) -> Option<crate::Space> {
        self.as_step().next_space(start)
    }

    fn as_cardinals(&self) -> impl IntoIterator<Item = Cardinal>;

    fn contains_cardinal(&self, cardinal: Cardinal) -> bool {
        self.as_cardinals().into_iter().any(|item| item == cardinal)
    }

    fn parse_step(step: step::Step) -> Option<Self>
    where
        Self: Sized;
}
