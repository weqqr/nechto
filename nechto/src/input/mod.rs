use std::collections::HashMap;

use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone)]
pub struct Action {
    name: String,
}

impl Action {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct InputHandler {
    keys_to_actions: HashMap<KeyCode, Action>,
    action_queue: Vec<Action>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keys_to_actions: HashMap::new(),
            action_queue: Vec::new(),
        }
    }

    pub fn add_action(&mut self, key: KeyCode, action: Action) {
        self.keys_to_actions.insert(key, action);
    }

    pub fn submit_key_event(&mut self, event: KeyEvent) {
        let PhysicalKey::Code(keycode) = event.physical_key else {
            return;
        };

        let Some(action) = self.keys_to_actions.get(&keycode) else {
            return;
        };

        self.action_queue.push(action.clone());
    }

    pub fn actions(&mut self) -> impl Iterator<Item = &Action> {
        self.action_queue.iter()
    }

    pub fn reset(&mut self) {
        self.action_queue.clear();
    }
}
