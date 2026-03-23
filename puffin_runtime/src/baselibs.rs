use crate::{value::{NativeFunction, Value}, vm::Vm};

pub fn define_print_function(vm: &mut Vm) {
    vm.add_global("print", NativeFunction::new(|runtime| {
        // Get value
        let value = runtime.get_local(-1)?;

        // Render value
        println!("{}", value);

        // Wait for user input
        runtime.poll()?;

        // Return null
        Ok(Value::Null)
    }, 1));
}
