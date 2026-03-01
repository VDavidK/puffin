use std::cell::RefMut;
use crate::value::Object;

pub trait Library {
    fn name() -> &'static str;
    fn create(lib: RefMut<Object>);
}