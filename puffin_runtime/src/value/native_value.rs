use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Display;
use std::sync::Arc;
use crate::RuntimeError;
use crate::value::{Value};
use crate::value::ops::{ValueDef, ValueTruthy};

pub type NativeValueType = NativeValue;

pub trait NativeValueTrait: Any + Display + std::fmt::Debug + 'static { }

#[derive(Debug, Clone)]
pub struct NativeValue(Arc<RefCell<dyn NativeValueTrait>>);


impl NativeValue {
    pub fn new<T: NativeValueTrait>(value: T) -> Self {
        value.into()
    }

    pub fn get<T: NativeValueTrait>(&self) -> Option<Ref<'_, T>> {
        Ref::filter_map(self.0.borrow(), |inner| (inner as &dyn Any).downcast_ref::<T>())
            .ok()
    }

    pub fn get_mut<T: NativeValueTrait>(&self) -> Option<RefMut<'_, T>> {
        RefMut::filter_map(self.0.borrow_mut(), |inner| (inner as &mut dyn Any).downcast_mut::<T>())
            .ok()
    }
}

impl<T> From<T> for NativeValue
where T : NativeValueTrait
{
    fn from(value: T) -> Self {
        NativeValue(Arc::new(RefCell::new(value)))
    }
}

impl TryFrom<Value> for NativeValueType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::NativeValue(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "nativevalue".to_owned() }),
        }
    }
}
impl From<NativeValue> for Value {
    fn from(value: NativeValue) -> Self {
        Value::NativeValue(value)
    }
}

impl PartialEq for NativeValue {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.0.as_ptr(), other.0.as_ptr())
    }
}

impl Display for NativeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.borrow()))
    }
}

impl ValueTruthy for NativeValueType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for NativeValueType {
    const TYPE_NAME: &'static str = "native_value";
}