use std::cell::RefCell;
use std::rc::Rc;
use crate::RuntimeError;
use crate::value::ops::{ValueAdd, ValueDef, ValueTruthy};
use crate::value::Value;

pub type StringType = Rc<RefCell<String>>;

impl From<StringType> for Value {
    fn from(value: StringType) -> Self {
        Value::String(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::String(Rc::new(RefCell::new(value.to_string())))
    }
}

impl TryFrom<Value> for StringType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "string".to_owned() }),
        }
    }
}

impl ValueAdd for StringType {
    fn try_add(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        Ok(Value::String(Rc::new(RefCell::new(format!("{}{}", self.borrow() ,rhs.to_owned())))))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(Rc::new(RefCell::new(value.to_owned())))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(Rc::new(RefCell::new(value)))
    }
}

impl ValueTruthy for StringType {
    fn truthy(&self) -> bool {
        !self.borrow().is_empty()
    }
}

impl ValueDef for StringType {
    const TYPE_NAME: &'static str = "string";
}