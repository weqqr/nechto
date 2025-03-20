mod object;

pub use self::object::{IntoObject, Object, ObjectData};

use crate::collections::Pool;
use crate::handle::Handle;

pub struct Realm {
    objects: Pool<Object>,
}

impl Realm {
    pub fn new() -> Self {
        Self {
            objects: Pool::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) -> Handle<Object> {
        // TODO: account for relationship data in object
        self.objects.insert(object)
    }
}
