use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::gpu::{self, ContextOptions};
use crate::vfs::VirtualFs;

pub struct Renderer {
    window: Window,
    vfs: Arc<VirtualFs>,

    ctx: gpu::Context,
    test_pipeline: gpu::Pipeline,
}

impl Renderer {
    pub fn new(window: Window, vfs: Arc<VirtualFs>) -> Self {
        let size = window.inner_size();
        let mut ctx = gpu::Context::new(
            window.window_handle().unwrap(),
            size.width,
            size.height,
            ContextOptions { enable_debug: true },
        );

        let shader = vfs.read("$build/world.spv").unwrap();

        let test_pipeline = ctx.create_pipeline(gpu::PipelineDescriptor {
            vertex_shader: shader.clone(),
            fragment_shader: shader,
        });

        Self {
            window,
            ctx,
            vfs,
            test_pipeline,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.ctx.resize_swapchain(size.width, size.height);
    }
}
