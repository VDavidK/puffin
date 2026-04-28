use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use crate::RuntimeError;
use crate::value::instance::InstanceType;
use crate::value::ops::ValueTruthy;
use crate::value::Value;

pub type ReactiveType = Rc<RefCell<Reactive>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactive(Value);

impl Reactive {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn set(&mut self, new_value: Value) {
        self.0 = new_value;
    }

    pub fn get(&self) -> &Value {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl TryFrom<Value> for ReactiveType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Reactive(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "reactive".to_owned() }),
        }
    }
}

impl From<Reactive> for Value {
    fn from(value: Reactive) -> Self {
        Value::Reactive(Rc::new(RefCell::new(value)))
    }
}

impl Display for Reactive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl ValueTruthy for ReactiveType {
    fn truthy(&self) -> bool {
        self.borrow().get().truthy()
    }
}
