use std::{any::Any, cell::{Ref, RefCell, RefMut}, fmt::Display, sync::Arc};


#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    UserValue(UserValueHandle),
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<UserValueHandle> for Value {
    fn from(value: UserValueHandle) -> Self {
        Self::UserValue(value)
    }
}

pub trait UserValue: Any + Display + std::fmt::Debug + 'static { }

#[derive(Debug, Clone)]
pub struct UserValueHandle(Arc<RefCell<dyn UserValue>>);


impl UserValueHandle {
    pub fn new<T: UserValue>(value: T) -> Self {
        Self(Arc::new(RefCell::new(value)))
    }

    pub fn get<'a, T: UserValue>(&'a self) -> Option<Ref<'a, T>> {
        Ref::filter_map(self.0.borrow(), |inner| (inner as &dyn Any).downcast_ref::<T>().into())
            .ok()
    }

    pub fn get_mut<'a, T: UserValue>(&'a self) -> Option<RefMut<'a, T>> {
        RefMut::filter_map(self.0.borrow_mut(), |inner| (inner as &mut dyn Any).downcast_mut::<T>().into())
            .ok()
    }

    pub fn unwrap<'a, T: UserValue>(&'a self) -> Ref<'a, T> {
        Ref::map(self.0.borrow(), |inner| (inner as &dyn Any).downcast_ref::<T>().unwrap())
    }

    pub fn unwrap_mut<'a, T: UserValue>(&'a self) -> RefMut<'a, T> {
        RefMut::map(self.0.borrow_mut(), |inner| (inner as &mut dyn Any).downcast_mut::<T>().unwrap())
    }
}

