use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{InstanceType, Value};
use crate::value::ops::{ValueDef, ValueTruthy};

pub type NativeFunctionType = Rc<RefCell<NativeFunction>>;

pub type NativeCallable = fn(runtime: &mut Runtime, argc: usize, this: Option<InstanceType>) -> Result<Value, RuntimeError>;

#[derive(Debug)]
pub struct NativeFunction {
    pub fun: NativeCallable,
    pub bound_value: Option<InstanceType>,
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<native_function()>"))
    }
}

impl NativeFunction {
    pub fn new(callable: NativeCallable) -> Self {
        Self {
            fun: callable,
            bound_value: None,
        }
    }

    pub fn bound_to(mut self, instance: InstanceType) -> Self {
        self.bind(instance);
        self
    }

    pub fn bind(&mut self, instance: InstanceType) {
        self.bound_value = Some(instance);
    }

    pub fn bound_copy(&self, instance: InstanceType) -> Self {
        Self {
            fun: self.fun,
            bound_value: Some(instance),
        }
    }
}

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Value::NativeFunction(Rc::new(RefCell::new(value)))
    }
}

impl TryFrom<Value> for NativeFunctionType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::NativeFunction(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "native_function".to_owned() }),
        }
    }
}

impl ValueTruthy for NativeFunctionType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for NativeFunctionType {
    const TYPE_NAME: &'static str = "native_function";
}