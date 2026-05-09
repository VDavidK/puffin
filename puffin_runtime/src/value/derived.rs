use std::fmt::{Debug, Display};
use std::rc::Rc;
use crate::RuntimeError;
use crate::value::Value;
use crate::value::ops::ValueTruthy;

pub type DerivedType = Derived;

#[derive(Clone)]
pub struct Derived {
    eval_fn: Rc<dyn Fn() -> Result<Value, RuntimeError>>,
    refs: Vec<Value>,
}

impl Derived {
    pub fn eval_or_null(&self) -> Value {
        (self.eval_fn)().unwrap_or(Value::Null)
    }

    pub fn eval(&self) -> Result<Value, RuntimeError> {
        (self.eval_fn)()
    }

    pub fn references(&self, value: &Value) -> bool {
        self.refs.contains(value) || self.refs
            .iter()
            .any(|r| match r {
                Value::Derived(derived) => derived.references(value),
                _ => false,
            })
    }
}

impl PartialEq for Derived {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(Rc::as_ptr(&self.eval_fn), Rc::as_ptr(&other.eval_fn))
        && self.refs.as_ptr() == other.refs.as_ptr()
    }
}

impl Debug for Derived {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Derived").finish()
    }
}

impl Display for Derived {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.eval_or_null()))
    }
}

impl ValueTruthy for DerivedType {
    fn truthy(&self) -> bool {
        self.eval_or_null().truthy().is_ok_and(|val| val)
    }
}

impl TryFrom<Value> for DerivedType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Derived(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "derived".to_owned() }),
        }
    }
}

impl From<Derived> for Value {
    fn from(value: Derived) -> Self {
        Value::Derived(value)
    }
}

pub fn derive_binary<F: Fn(&Value, &Value) -> Result<Value, RuntimeError> + 'static>(lhs: &Value, rhs: &Value, operation: F) -> Result<Value, RuntimeError> {
    let lhs = lhs.clone();
    let rhs = rhs.clone();

    let refs = get_valid_references([lhs.clone(), rhs.clone()]);

    let closure = move || -> Result<Value, RuntimeError> {
        let lhs = lhs.eval()?;
        let rhs = rhs.eval()?;
        operation(&lhs, &rhs)
    };

    let derived = Derived {
        eval_fn: Rc::new(closure),
        refs,
    };

    Ok(Value::Derived(derived))
}

pub fn derive_unary<F: Fn(&Value) -> Result<Value, RuntimeError> + 'static>(val: &Value, operation: F) -> Result<Value, RuntimeError> {
    let val = val.clone();
    let refs = get_valid_references([val.clone()]);

    let closure = move || -> Result<Value, RuntimeError> {
        operation(&val)
    };

    let derived = Derived {
        eval_fn: Rc::new(closure),
        refs,
    };

    Ok(Value::Derived(derived))
}

fn get_valid_references(values: impl IntoIterator<Item = Value>) -> Vec<Value> {
    values
        .into_iter()
        .filter(|value| value.is_reactive())
        .collect::<Vec<Value>>()
}
