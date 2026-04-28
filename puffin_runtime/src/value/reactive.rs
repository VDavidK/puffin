use std::fmt::Display;
use serde_derive::{Serialize, Deserialize};
use crate::value::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactive(Value);

impl Reactive {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn set(&mut self, new_value: Value) {
        self.0 = new_value;
    }

    pub fn get(&self) -> &Value {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Value {
        &mut self.0
    }
}

impl Display for Reactive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
