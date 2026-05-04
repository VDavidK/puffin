use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::event::Event;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{Value, ClassType, IntType, Reactive};
use crate::value::ops::{ValueDef, ValueTruthy};

pub type InstanceType = Rc<RefCell<Instance>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    class: ClassType,
    fields: HashMap<String, Value>,
    event_handlers: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: ClassType) -> Self {
        Self {
            class,
            fields: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, name: impl Into<String>, value: impl Into<Value>) -> Result<(), RuntimeError> {
        let name = name.into();

        if let Some(inner) = self.fields.get(&name) {
            match inner {
                Value::Reactive(inner) => {
                    Reactive::set(inner.clone(), value.into())?;
                    return Ok(());
                }
                _ => (),
            }
        }

        self.fields.insert(name, value.into());
        Ok(())
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }

    pub fn set_event_handler(&mut self, name: impl Into<String>, handler: impl Into<Value>) {
        self.event_handlers.insert(name.into(), handler.into());
    }
    
    pub fn get_class(&self) -> ClassType {
        self.class.clone()
    }

    pub fn get_handler(&self, name: impl AsRef<str>) -> Option<Value> {
        self.event_handlers.get(name.as_ref()).cloned()
    }

    pub fn dispatch_event(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        match event {
            Event::KeyPress(c) => {
                if let Some(handler) = self.event_handlers.get("onkey") {
                    runtime.call(handler.to_owned(), &[(*c).into()])?;
                }
            }
        }

        Ok(())
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
impl From<InstanceType> for Value {
    fn from(value: InstanceType) -> Self {
        Value::Instance(value)
    }
}

impl From<Instance> for Value {
    fn from(value: Instance) -> Self {
        Value::Instance(Rc::new(RefCell::new(value)))
    }
}

pub fn new_instance(class: ClassType) -> InstanceType {
    let mut instance = Instance::new(class.clone());

    let class = class.borrow();
    let fields = class.get_fields();

    for (k, v) in fields.iter() {
        instance.set_field(k, v.clone());
    }

    let instance = Rc::new(RefCell::new(instance));

    for (k, v) in &class.methods {
        match v {
            Value::Function(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_field(k, method);
            }
            Value::NativeFunction(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_field(k, method);
            }
            _ => unreachable!("Class method needs to be of type function"),
        }
    }

    for (k, v) in &class.handlers {
        match v {
            Value::Function(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_event_handler(k, method);
            }
            Value::NativeFunction(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_event_handler(k, method);
            }
            _ => unreachable!("Class event handler needs to be of type function"),
        }
    }

    instance
}

impl ValueTruthy for InstanceType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for InstanceType {
    const TYPE_NAME: &'static str = "instance";
}