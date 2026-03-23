use std::fmt::Display;
use std::rc::Rc;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{Value, NativeFunctionType};

pub type NativeCallable = fn(runtime: &mut Runtime) -> Result<Value, RuntimeError>;

#[derive(Debug)]
pub struct NativeFunction {
    pub fun: NativeCallable,
    pub arity: usize,
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<native_function({})>", self.arity))
    }
}

impl NativeFunction {
    pub fn new(callable: NativeCallable, num_args: usize) -> Self {
        Self {
            fun: callable,
            arity: num_args,
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
