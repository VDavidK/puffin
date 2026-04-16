use std::fmt::Display;
use std::rc::Rc;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{Value, NativeFunctionType};

pub type NativeCallable = fn(runtime: &mut Runtime, argc: usize) -> Result<Value, RuntimeError>;

#[derive(Debug)]
pub struct NativeFunction {
    pub fun: NativeCallable,
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
        }
    }
}

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Value::NativeFunction(Rc::new(value))
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
