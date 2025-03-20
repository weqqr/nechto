use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::gpu::{self, ContextOptions};

pub struct Renderer {
    window: Window,
    ctx: gpu::Context,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();
        let ctx = gpu::Context::new(
            window.window_handle().unwrap(),
            size.width,
            size.height,
            ContextOptions::default(),
        );

        Self { window, ctx }
    }
}
