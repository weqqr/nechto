use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};

use crate::input::{Action, InputHandler};
use crate::render::Renderer;
use crate::vfs::VirtualFs;

pub struct Resources {
    renderer: Option<Renderer>,
    input_handler: InputHandler,
    vfs: Arc<VirtualFs>,
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
                if let Some(renderer) = &mut resources.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                resources.input_handler.submit_key_event(event);
            }
            _ => {}
        }
    }

    fn on_update(&mut self, event_loop: &ActiveEventLoop, resources: &mut Resources) {
        for action in resources.input_handler.actions() {
            if action.name() == "quit" {
                event_loop.exit();
            }
        }

        resources.input_handler.reset();
    }

    fn on_render(&mut self, resources: &mut Resources) {
        if let Some(renderer) = &mut resources.renderer {
            renderer.render();
        }
    }
}

pub struct Runtime {
    event_handler: EventHandler,
    resources: Resources,
}

impl Runtime {
    pub fn new() -> Self {
        let mut input_handler = InputHandler::new();

        input_handler.add_action(KeyCode::Escape, Action::new("quit"));

        let mut vfs = VirtualFs::new();
        vfs.add_search_path("$build", "build".into());
        vfs.add_search_path("$data", "data".into());

        let vfs = Arc::new(vfs);

        Self {
            event_handler: EventHandler::new(),
            resources: Resources {
                renderer: None,
                input_handler,
                vfs,
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

        let renderer = Renderer::new(window, Arc::clone(&self.resources.vfs));
        self.resources.renderer = Some(renderer);
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
        self.event_handler
            .on_update(event_loop, &mut self.resources);
        self.event_handler.on_render(&mut self.resources);
    }
}
