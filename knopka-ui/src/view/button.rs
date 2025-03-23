use crate::value::{IntoValue, Value};
use crate::view::View;

pub struct Button {
    text: Value<String>,

    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(text: impl IntoValue<String>) -> Self {
        Self {
            text: text.into_value(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl View for Button {
    fn update(&mut self) {
        todo!()
    }

    fn render(&self, painter: &dyn super::Painter) {
        todo!()
    }
}
