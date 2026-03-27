use crate::{runtime::Runtime, value::{NativeFunction, Value}};

pub fn define_print_function(runtime: &mut Runtime) {
    runtime.add_global("print", NativeFunction::new(|runtime| {
        // Get value
        let value = runtime.get_local(-1)?;

        // Render value
        // TODO: Fix
        println!("{}", value);

        // Wait for user input
        runtime.poll()?;

        // Return null
        Ok(Value::Null)
    }, 1));
}
