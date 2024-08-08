use super::{PieceComponent as Component, PieceComponents as Components};

#[derive(Default, Debug)]
pub struct PieceComponentsBuilder {
    inner: Vec<Component>,
}

impl PieceComponentsBuilder {
    pub fn build(self) -> Components {
        Components { inner: self.inner }
    }

    pub fn new(inner: impl FnOnce(&mut Self) -> &mut Self) -> Self {
        let mut new = Self::default();
        inner(&mut new);
        new
    }

    pub fn add_component(&mut self, component: Component) -> &mut Self {
        self.inner.push(component);
        self
    }

    pub fn add_components(&mut self, components: impl IntoIterator<Item = Component>) -> &mut Self {
        self.inner.extend(components);
        self
    }
}

impl Extend<Component> for PieceComponentsBuilder {
    fn extend<T: IntoIterator<Item = Component>>(&mut self, iter: T) {
        self.add_components(iter);
    }
}
