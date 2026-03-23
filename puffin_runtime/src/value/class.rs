use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use crate::{RuntimeError};
use crate::value::{Value, ClassType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub constructor: Option<Value>,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Value>,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            constructor: None,
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn set_constructor(&mut self, value: Value) {
        self.constructor = Some(value);
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Class [{}]", self.name))
    }
}

impl TryFrom<Value> for ClassType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Class(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "class".to_owned() }),
        }
    }
}

pub fn new_class(name: impl Into<String>) -> ClassType {
    Rc::new(RefCell::new(Class::new(name)))
}
