use ash::vk;
use glam::{IVec2, UVec2};

#[derive(Clone, Copy)]
pub struct Rect2D {
    pub offset: IVec2,
    pub extent: UVec2,
}

#[derive(Clone, Copy)]
pub struct ImageView {
    pub(super) image_view: vk::ImageView,
}
