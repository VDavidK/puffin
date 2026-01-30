use crate::{program::Program, Value};


#[derive(Debug, Clone)]
pub struct Vm<'a> {
    program: &'a Program,
    stack: Vec<Value>,
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            stack: vec![],
        }
    }

    pub fn push_value<T: Into<Value>>(&mut self, value: T) {
        self.stack.push(value.into());
    }
}

