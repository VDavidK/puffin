use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{InstanceType, Value};

pub enum Event {
    KeyPress(char),
}

impl Event {
    pub fn dispatch(&self, runtime: &mut Runtime, instance: InstanceType) -> Result<(), RuntimeError> {
        let handler = match self {
            Event::KeyPress(c) => instance.borrow().get_handler("onkey"),
        };

        if handler.is_none() {
            return Ok(())
        }

        match self {
            Event::KeyPress(c) => {
                runtime.call(handler.unwrap(), &[(*c).into()])?;
            }
        }

        Ok(())
    }
}
