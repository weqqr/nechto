use std::marker::PhantomData;

use crate::view::View;

pub struct Scope {
    cx: ScopeContext,
    view: Box<dyn View>,
}

impl Scope {
    pub fn new<V: View, F: Fn(&mut ScopeContext) -> V>(f: F) -> Self {
        let mut cx = ScopeContext::new();

        let view = f(&mut cx);

        Self {
            cx,
            view: Box::new(view),
        }
    }
}

impl View for Scope {
    fn update(&mut self) {
        self.view.update();
    }

    fn render(&self, painter: &dyn super::Painter) {
        self.view.render(painter);
    }
}

pub struct ScopeContext {}

impl ScopeContext {
    fn new() -> Self {
        Self {}
    }

    pub fn state<T>(&self, initial: T) -> React<T> {
        React { _pd: PhantomData }
    }
}

pub struct React<T> {
    _pd: PhantomData<fn() -> *const T>,
}

impl<T> Copy for React<T> {}
impl<T> Clone for React<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: std::fmt::Display> std::fmt::Display for React<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
