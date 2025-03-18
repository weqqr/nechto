pub mod allocator;

use std::marker::PhantomData;

pub struct Handle<T> {
    value: u64,
    _pd: PhantomData<fn(&T)>,
}

impl<T> Handle<T> {
    pub const NIL: Self = Self::new(0);

    pub const fn new(value: u64) -> Self {
        Self {
            value,
            _pd: PhantomData,
        }
    }

    pub const fn from_usize(value: usize) -> Self {
        Self::new(value as u64)
    }

    pub const fn to_usize(&self) -> usize {
        self.value as usize
    }
}
