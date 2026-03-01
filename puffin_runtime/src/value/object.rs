use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::{RuntimeError, Value};
use crate::value::ObjectType;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Object {
    fields: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_field(&mut self, name: impl AsRef<str>, value: impl Into<Value>) {
        self.fields.insert(name.as_ref().to_owned(), value.into());
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Object")
    }
}

impl TryFrom<Value> for ObjectType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "object".to_owned() }),
        }
    }
}

pub fn new_object() -> ObjectType {
    Rc::new(RefCell::new(Object::new()))
}
