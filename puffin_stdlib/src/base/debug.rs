use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{NativeFunction, Value};

pub fn define_print_function(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    runtime.add_global("print", NativeFunction::new(|runtime, _argc, _| {
        // Get value
        let value = runtime.get_local(-1)?;

        log::info!("{}", value);

        // Return null
        Ok(Value::Null)
    }))?;
    Ok(())
}

pub fn define_exit_function(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    runtime.add_global("exit", NativeFunction::new(|runtime, _argc, _| {
        runtime.exit();

        // Return null
        Ok(Value::Null)
    }))?;
    Ok(())
}