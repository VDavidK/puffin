use crate::RuntimeError;
use crate::value::ops::{ValueDef, ValueTruthy};
use crate::value::Value;

pub type BoolType = bool;


impl From<BoolType> for Value {
    fn from(value: BoolType) -> Self {
        Value::Bool(value)
    }
}

impl TryFrom<Value> for BoolType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "bool".to_owned() }),
        }
    }
}

impl ValueTruthy for BoolType {
    fn truthy(&self) -> bool {
        *self
    }
}

impl ValueDef for BoolType {
    const TYPE_NAME: &'static str = "bool";
}