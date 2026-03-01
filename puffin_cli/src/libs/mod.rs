use std::cell::RefMut;
use puffin_runtime::library::Library;
use puffin_runtime::Value;
use puffin_runtime::value::{NativeFunction, Object};

pub struct CoreLib;

impl Library for CoreLib {
    fn name() -> &'static str { "core" }

    fn create(mut lib: RefMut<Object>) {
        lib.set_field("print", NativeFunction::new(|runtime| {
            // Get value
            let value = runtime.get_local(-1)?;

            // Render value
            runtime.render(value.clone())?;

            // Wait for user input
            runtime.poll()?;

            // Return null
            Ok(Value::Null)
        }, 1));
    }
}
