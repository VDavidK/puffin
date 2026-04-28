use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::{RuntimeError};
use crate::value::{FunctionType, Value};
use crate::value::instance::InstanceType;
use crate::value::ops::{ValueDef, ValueTruthy};

pub type ModuleType = Rc<RefCell<Module>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    name: String,
    items: HashMap<String, Value>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            items: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_item(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        self.items.insert(name.into(), value.into());
    }

    pub fn get_item(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.items.get(name.as_ref())
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Module [{}]", self.name))
    }
}

impl From<ModuleType> for Value {
    fn from(value: ModuleType) -> Self {
        Value::Module(value)
    }
}

impl TryFrom<Value> for ModuleType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Module(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "module".to_owned() }),
        }
    }
}

pub fn new_module(name: impl Into<String>) -> ModuleType {
    Rc::new(RefCell::new(Module::new(name)))
}

impl ValueTruthy for ModuleType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for ModuleType {
    const TYPE_NAME: &'static str = "module";
}