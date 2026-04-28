use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use crate::{RuntimeError};
use crate::value::{Value};
use crate::value::instance::InstanceType;
use crate::value::ops::ValueTruthy;

pub type ClassType = Rc<RefCell<Class>>;

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

    pub fn set_constructor(&mut self, value: impl Into<Value>) {
        self.constructor = Some(value.into());
    }

    pub fn get_constructor(&self) -> Option<&Value> {
        self.constructor.as_ref()
    }

    pub fn set_field(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.fields.insert(name.into(), value.into());
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }

    pub fn get_method(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.methods.get(name.as_ref())
    }

    pub fn set_method(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.methods.insert(name.into(), value.into());
    }

    pub fn get_fields(&self) -> &HashMap<String, Value> {
        &self.fields
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

impl From<ClassType> for Value {
    fn from(value: ClassType) -> Self {
        Value::Class(value)
    }
}

impl From<Class> for Value {
    fn from(value: Class) -> Self {
        Value::Class(Rc::new(RefCell::new(value)))
    }
}

pub fn new_class(name: impl Into<String>) -> ClassType {
    Rc::new(RefCell::new(Class::new(name)))
}

impl ValueTruthy for ClassType {
    fn truthy(&self) -> bool {
        true
    }
}
