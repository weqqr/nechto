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

pub enum MemoryType {
    DeviceLocal,
    HostVisible,
}

pub struct BufferDescriptor {
    pub memory_type: MemoryType,
    pub size: u64,
    pub usage_flags: u64,
}
