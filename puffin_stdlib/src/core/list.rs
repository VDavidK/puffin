use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;
use std::fs::OpenOptions;
use std::io::Write;

struct ListSystem;

impl Declaration for ListSystem {
    const NAME: &'static str = "list";

    fn declare(module: &mut Module) {
        module.set_item("push", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_list()?;

            let value = runtime.get_local(-1)?.eval()?;
            list.borrow_mut().push(value.to_owned());
            Ok(value)
        }));
        module.set_item("push_front", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_list()?;

            let value = runtime.get_local(-1)?.eval()?;
            list.borrow_mut().insert(0, value.to_owned());
            Ok(value)
        }));
        module.set_item("pop", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_list()?;
            Ok(list.borrow_mut().pop().unwrap_or(Value::Null))
        }));
        module.set_item("pop_front", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_list()?;
            if list.borrow().is_empty() {
                Ok(Value::Null)
            } else {
                Ok(list.borrow_mut().remove(0))
            }
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(ListSystem::NAME);
    ListSystem::declare(&mut module);

    module
}
