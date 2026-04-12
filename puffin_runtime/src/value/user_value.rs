use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Display;
use std::sync::Arc;
use crate::RuntimeError;
use crate::value::{NativeValueType, Value};

pub trait UserValueTrait: Any + Display + std::fmt::Debug + 'static { }

#[derive(Debug, Clone)]
pub struct NativeValue(Arc<RefCell<dyn UserValueTrait>>);


impl NativeValue {
    pub fn new<T: UserValueTrait>(value: T) -> Self {
        value.into()
    }

    pub fn get<T: UserValueTrait>(&self) -> Option<Ref<T>> {
        Ref::filter_map(self.0.borrow(), |inner| (inner as &dyn Any).downcast_ref::<T>().into())
            .ok()
    }

    pub fn get_mut<T: UserValueTrait>(&self) -> Option<RefMut<T>> {
        RefMut::filter_map(self.0.borrow_mut(), |inner| (inner as &mut dyn Any).downcast_mut::<T>().into())
            .ok()
    }

    pub fn unwrap<T: UserValueTrait>(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |inner| (inner as &dyn Any).downcast_ref::<T>().unwrap())
    }

    pub fn unwrap_mut<T: UserValueTrait>(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |inner| (inner as &mut dyn Any).downcast_mut::<T>().unwrap())
    }
}

impl<T> From<T> for NativeValue
where T : UserValueTrait {
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

impl PartialEq for NativeValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl Display for NativeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.borrow()))
    }
}