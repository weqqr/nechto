use std::sync::Arc;

use ash::vk;
use glam::{IVec2, UVec2};
use winit::dpi::PhysicalSize;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::config::RenderConfig;
use crate::gpu::{self, ContextOptions};
use crate::vfs::VirtualFs;

pub struct Renderer {
    window: Window,
    vfs: Arc<VirtualFs>,

    ctx: gpu::Context,
    test_pipeline: gpu::Pipeline,
}

// FIXME: Implement GC for gpu objects and remove Drop impl for renderer
impl Drop for Renderer {
    fn drop(&mut self) {
        self.ctx.destroy_pipeline(&mut self.test_pipeline);
    }
}

impl Renderer {
    pub fn new(window: Window, vfs: Arc<VirtualFs>, config: RenderConfig) -> Self {
        let size = window.inner_size();
        let mut ctx = gpu::Context::new(
            window.window_handle().unwrap(),
            size.width,
            size.height,
            ContextOptions {
                enable_debug: config.vulkan_enable_debug,
            },
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

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.ctx.resize_swapchain(size.width, size.height);
    }

    pub fn render(&mut self) {
        let size = self.window.inner_size();

        let mut frame = self.ctx.begin_frame();
        let target = frame.image_view();

        let cmd = frame.command_buffer();

        cmd.begin_rendering(
            target,
            gpu::Rect2D {
                offset: IVec2::new(0, 0),
                extent: UVec2::new(size.width, size.height),
            },
        );

        cmd.end_rendering();

        self.ctx.end_frame(frame);
    }
}
