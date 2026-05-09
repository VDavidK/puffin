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
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: DictionaryType::type_name().to_owned() }),
        }
    }
}

impl From<HashMap<Value, Value>> for Value {
    fn from(value: HashMap<Value, Value>) -> Self {
        Value::Dictionary(Rc::new(RefCell::new(value)))
    }
}

impl From<Vec<(Value, Value)>> for Value {
    fn from(value: Vec<(Value, Value)>) -> Self {
        Value::Dictionary(Rc::new(RefCell::new(value.into_iter().collect())))
    }
}

impl From<DictionaryType> for Value {
    fn from(value: DictionaryType) -> Self {
        Value::Dictionary(value)
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

#[macro_export]
macro_rules! make_dict {
    () => { std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::<crate::value::Value, crate::value::Value>::new())) };
    ($($x:expr => $y:expr),+ $(,)?) => {
        std::rc::Rc::new(std::cell::RefCell::new(
            vec![$((crate::value::Value::from($x), crate::value::Value::from($y))),+]
                .into_iter()
                .collect::<std::collections::HashMap<crate::value::Value, crate::value::Value>>()
        ))
    }
}

#[macro_export]
macro_rules! make_dict_value {
    () => { Value::Dictionary(crate::make_dict!()) };
    ($($x:expr => $y:expr),+ $(,)?) => {
        Value::Dictionary(crate::make_dict![$($x => $y),*])
    }
}

#[macro_export]
macro_rules! make_fields {
    () => { Value::Dictionary(vec![].into()) };
    ($($x:ident: $y:expr),+ $(,)?) => {
        crate::value::Value::from(
            vec![$((crate::value::Value::from(stringify!($x)), crate::value::Value::from($y))),+]
                .into_iter()
                .collect::<std::collections::HashMap<crate::value::Value, crate::value::Value>>()
        )
    }
}
