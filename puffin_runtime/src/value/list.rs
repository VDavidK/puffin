use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::RuntimeError;
use crate::value::ops::ValueTruthy;
use crate::value::Value;

pub type ListType = Rc<RefCell<Vec<Value>>>;

impl TryFrom<Value> for ListType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "list".to_owned() }),
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(Rc::new(RefCell::new(value)))
    }
}

pub struct ListDisplay<'a>(pub &'a ListType);

impl<'a> Display for ListDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0.borrow().iter()
            .map(Value::to_string)
            .collect::<Vec<String>>()
            .join(", ");

        f.write_fmt(format_args!("[{}]", inner))
    }
}

impl ValueTruthy for ListType {
    fn truthy(&self) -> bool {
        true
    }
}