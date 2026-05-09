use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use crate::RuntimeError;
use crate::value::ops::ValueTruthy;
use crate::value::Value;

pub type ReactiveType = Rc<RefCell<Reactive>>;

#[derive(Debug, Clone)]
pub struct Reactive(Value);

impl Reactive {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn set(this: ReactiveType, new_value: Value) -> Result<(), RuntimeError> {
        let value = if new_value.references(&Value::Reactive(this.clone())) {
            new_value.eval()?
        } else {
            new_value
        };

        this.borrow_mut().0 = value;
        Ok(())
    }

    pub fn get(&self) -> &Value {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl TryFrom<Value> for ReactiveType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Reactive(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "reactive".to_owned() }),
        }
    }
}

impl From<Reactive> for Value {
    fn from(value: Reactive) -> Self {
        Value::Reactive(Rc::new(RefCell::new(value)))
    }
}

impl Display for Reactive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl ValueTruthy for ReactiveType {
    fn truthy(&self) -> bool {
        self.borrow().get().truthy().is_ok_and(|val| val)
    }
}
