use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::RuntimeError;
use crate::value::{Value, ClassType, InstanceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    class: ClassType,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: ClassType) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, name: impl AsRef<str>, value: impl Into<Value>) {
        self.fields.insert(name.as_ref().to_owned(), value.into());
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Object [{}]", self.class.borrow().name))
    }
}

impl TryFrom<Value> for InstanceType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Instance(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "instance".to_owned() }),
        }
    }
}

pub fn new_instance(class: ClassType) -> InstanceType {
    let mut instance = Instance::new(class.clone());

    let class = class.borrow();
    let fields = class.get_fields();

    for (k, v) in fields.iter() {
        instance.set_field(k, v.clone());
    }

    Rc::new(RefCell::new(instance))
}
