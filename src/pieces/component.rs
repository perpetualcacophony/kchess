use std::any::Any;

mod builder;
pub use builder::PieceComponentsBuilder as Builder;

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
