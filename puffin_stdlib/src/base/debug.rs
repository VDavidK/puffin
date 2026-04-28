use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{NativeFunction, Value};

pub fn define_print_function(runtime: &mut Runtime) {
    runtime.add_global("print", NativeFunction::new(|runtime, argc, _| {
        // Get value
        let value = runtime.get_local(-1)?;

        log::info!("{}", value);

        // Return null
        Ok(Value::Null)
    }));
}

pub fn define_exit_function(runtime: &mut Runtime) {
    runtime.add_global("exit", NativeFunction::new(|runtime, argc, _| {
        runtime.exit();

        // Return null
        Ok(Value::Null)
    }));
}