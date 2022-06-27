use crate::component::Component;

#[derive(Clone)]
pub struct Namespace {
    pub name: String,
    pub components: Vec<Component>
}

impl Namespace {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), components: Vec::new() }
    }

    pub fn add_component(&mut self, component: Component) -> &mut Self {
        self.components.push(component);
        self
    }
}