pub mod button;
pub mod layout;
pub mod scope;
pub mod text;

pub use self::button::*;
pub use self::layout::*;
pub use self::scope::*;
pub use self::text::*;

pub trait View: 'static {
    fn update(&mut self);
    fn render(&self, painter: &dyn Painter);
}

pub trait Painter {
    fn draw_rect(&self);
}
