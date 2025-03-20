use crate::handle::Handle;

pub struct Pool<T> {
    items: Vec<Option<T>>,
    free_indexes: Vec<usize>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            free_indexes: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        if let Some(index) = self.free_indexes.pop() {
            self.items[index] = Some(value);
            Handle::from_usize(index)
        } else {
            let index = self.items.len();
            self.items.push(Some(value));
            Handle::from_usize(index)
        }
    }

    pub fn remove(&mut self, handle: Handle<T>) {
        self.free_indexes.push(handle.to_usize());
        self.items[handle.to_usize()] = None;
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.items.get(handle.to_usize()).and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.items
            .get_mut(handle.to_usize())
            .and_then(|x| x.as_mut())
    }
}
