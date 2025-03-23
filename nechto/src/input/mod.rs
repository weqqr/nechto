use std::collections::{HashMap, HashSet};

use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, PartialEq, Eq, Hash)]
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
    active_actions: HashSet<Action>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keys_to_actions: HashMap::new(),
            action_queue: Vec::new(),
            active_actions: HashSet::new(),
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

        match event.state {
            ElementState::Pressed => {
                self.active_actions.insert(action.clone());
            },
            ElementState::Released => {
                self.active_actions.remove(action);
            },
        }

        self.action_queue.push(action.clone());
    }

    pub fn actions(&mut self) -> impl Iterator<Item = &Action> {
        self.action_queue.iter()
    }

    pub fn reset(&mut self) {
        self.action_queue.clear();
    }

    pub fn is_action_active(&self, action: &Action) -> bool {
        self.active_actions.contains(action)
    }
}
