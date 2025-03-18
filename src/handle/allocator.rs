use std::marker::PhantomData;

use crate::handle::Handle;

pub struct HandleAllocator<T> {
    last_index: u64,
    _pd: PhantomData<fn(&T)>,
}

impl<T> HandleAllocator<T> {
    pub fn new() -> Self {
        Self {
            last_index: 0,
            _pd: PhantomData,
        }
    }

    pub fn allocate_handle(&mut self) -> Handle<T> {
        self.last_index += 1;
        Handle::new(self.last_index)
    }
}
