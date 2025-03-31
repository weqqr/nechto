use crate::runtime::Resources;

#[allow(unused_variables)]
pub trait App: 'static {
    fn init(&mut self, resources: &mut Resources) {}
    fn update(&mut self, resources: &mut Resources) {}
}
