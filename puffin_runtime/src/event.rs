use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::InstanceType;

pub enum Event {
    KeyPress(char),
}

impl Event {
    pub fn dispatch(&self, runtime: &mut Runtime, instance: InstanceType) -> Result<(), RuntimeError> {
        let handler = match self {
            Event::KeyPress(_c) => instance.borrow().get_handler("onkey"),
        };
        if let Some(h) = handler {
            match self {
                Event::KeyPress(c) => {
                    runtime.call(h, &[(*c).into()])?;
                }
            }
        }

        Ok(())
    }
}
