use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::render::Renderer;

pub struct Resources {
    window: Option<Renderer>,
}

pub struct EventHandler {}

impl EventHandler {
    fn new() -> Self {
        Self {}
    }

    fn on_window_event(
        &self,
        resources: &mut Resources,
        event_loop: &ActiveEventLoop,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                // resize renderer
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // submit to the input handler
            }
            _ => {}
        }
    }

    fn on_render(&mut self, resources: &mut Resources) {
        // render
    }
}

pub struct Runtime {
    event_handler: EventHandler,
    resources: Resources,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            event_handler: EventHandler::new(),
            resources: Resources {
                window: None,
            },
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for Runtime {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = event_loop.create_window(window_attributes).unwrap();

        let renderer = Renderer::new(window);
        self.resources.window = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.event_handler
            .on_window_event(&mut self.resources, event_loop, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.event_handler.on_render(&mut self.resources);
    }
}
