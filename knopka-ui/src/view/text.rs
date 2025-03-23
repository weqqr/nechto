use crate::value::{IntoValue, Value};
use crate::view::View;

pub struct Text {
    text: Value<String>,
}

impl Text {
    pub fn new(text: impl IntoValue<String>) -> Self {
        Self {
            text: text.into_value(),
        }
    }
}

impl View for Text {
    fn update(&mut self) {
        todo!()
    }

    fn render(&self, painter: &dyn super::Painter) {
        todo!()
    }
}
