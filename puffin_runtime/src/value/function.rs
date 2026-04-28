use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::{Chunk, RuntimeError};
use crate::value::{Value, FunctionType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub chunk: Rc<Chunk>,
    pub identifier: String,
    pub arity: usize,
    pub bound_value: Value,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<function {}({})>", self.identifier, self.arity))
    }
}

impl TryFrom<Value> for FunctionType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Function(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "function".to_owned() }),
        }
    }
}

impl From<Function> for Value {
    fn from(function: Function) -> Self {
        Value::Function(Rc::new(function))
    }
}
