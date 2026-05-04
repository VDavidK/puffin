use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use crate::RuntimeError;
use crate::value::ops::{ValueDef, ValueTruthy};
use crate::value::Value;

pub type DictionaryType = Rc<RefCell<HashMap<Value, Value>>>;

impl TryFrom<Value> for DictionaryType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dictionary(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "list".to_owned() }),
        }
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(value: HashMap<Value, Value>) -> Self {
        Value::Dictionary(Rc::new(RefCell::new(value)))
    }
}

pub struct DictionaryDisplay<'a>(pub &'a DictionaryType);

impl<'a> Display for DictionaryDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0.borrow().iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join(", ");

        f.write_fmt(format_args!("{{{}}}", inner))
    }
}

impl ValueTruthy for DictionaryType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for DictionaryType {
    const TYPE_NAME: &'static str = "dictionary";
}