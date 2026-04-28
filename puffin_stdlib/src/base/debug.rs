use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{NativeFunction, Value};

pub fn define_print_function(runtime: &mut Runtime) {
    runtime.add_global("print", NativeFunction::new(|runtime, argc| {
        // Get value
        let value = runtime.get_local(-1)?;

        log::info!("{}", value);

        // Wait for user input
        runtime.poll()?;

        // Return null
        Ok(Value::Null)
    }));
}