use crate::handle::Handle;

pub struct Object {
    data: Box<dyn ObjectData>,
    parent: Handle<Object>,
    children: Vec<Handle<Object>>,
}

pub trait ObjectData: 'static {
    fn name(&self) -> &'static str;
}

impl Object {
    pub fn new(data: impl ObjectData) -> Self {
        Object {
            data: Box::new(data),
            parent: Handle::NIL,
            children: Vec::new(),
        }
    }

    pub fn with_parent(mut self, parent: Handle<Object>) -> Self {
        self.parent = parent;
        self
    }

    pub fn with_child(mut self, child: Handle<Object>) -> Self {
        self.children.push(child);
        self
    }
}

pub trait IntoObject: ObjectData + Sized + 'static {
    fn into_object(self) -> Object {
        Object::new(self)
    }
}
