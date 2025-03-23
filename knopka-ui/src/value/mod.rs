pub enum Value<T> {
    Constant(T),
    Dynamic(Box<dyn DynamicValue<T>>),
}

impl<T: Clone + 'static> Value<T> {
    fn get(&self) -> T {
        match self {
            Value::Constant(constant_value) => constant_value.clone(),
            Value::Dynamic(dynamic_value) => dynamic_value.get(),
        }
    }
}

pub enum ValueMut<T> {
    Dynamic(Box<dyn DynamicValueMut<T>>),
}

pub trait IntoValue<T> {
    fn into_value(self) -> Value<T>;
}

impl IntoValue<String> for String {
    fn into_value(self) -> Value<String> {
        Value::Constant(self)
    }
}

impl IntoValue<String> for &str {
    fn into_value(self) -> Value<String> {
        Value::Constant(self.to_string())
    }
}

impl<T, V: DynamicValue<T>> IntoValue<T> for V {
    fn into_value(self) -> Value<T> {
        Value::Dynamic(Box::new(self))
    }
}

// DynamicValue

pub trait DynamicValue<T>: 'static {
    fn get(&self) -> T;
}

pub trait DynamicValueMut<T>: DynamicValue<T> {
    fn set(&self, value: T);
}

impl<T, F: Fn() -> T + 'static> DynamicValue<T> for F {
    fn get(&self) -> T {
        self()
    }
}
