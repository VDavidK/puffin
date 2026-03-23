use std::cell::RefMut;
use crate::value::Instance;

pub trait Library {
    fn name() -> &'static str;
    fn create(lib: RefMut<Instance>);
}