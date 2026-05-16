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
            let value = runtime.get_local(-1)?
                .to_owned();
            let list = runtime.get_local(-2)?
                .to_owned()
                .take_list()?;

            list.borrow_mut().push(value.to_owned());
            Ok(value)
        }));
        module.set_item("push_front", NativeFunction::new(|runtime, _, _| {
            let value = runtime.get_local(-1)?
                .to_owned();
            let list = runtime.get_local(-2)?
                .to_owned()
                .take_list()?;

            list.borrow_mut().insert(0, value.to_owned());
            Ok(value)
        }));
        module.set_item("pop", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-1)?
                .to_owned()
                .take_list()?;
            Ok(list.borrow_mut().pop().unwrap_or(Value::Null))
        }));
        module.set_item("pop_front", NativeFunction::new(|runtime, _, _| {
            let list = runtime.get_local(-1)?
                .to_owned()
                .take_list()?;
            if list.borrow().is_empty() {
                Ok(Value::Null)
            } else {
                Ok(list.borrow_mut().remove(0))
            }
        }));
        // list.replace(<list>, <value>, <idx>)
        module.set_item("replace", NativeFunction::new(|runtime, _, _| {
            let idx = runtime.get_local(-1)?
                .to_owned()
                .take_int()? as usize;
            let value = runtime.get_local(-2)?
                .to_owned();
            let list = runtime.get_local(-3)?
                .to_owned()
                .take_list()?;
            if (idx as isize) < 0 || list.borrow().len() - 1 < idx {
                Err(RuntimeError::IndexOutOfBounds { index: idx, size: list.borrow().len() })?;
            }
            list.borrow_mut().remove(idx);
            list.borrow_mut().insert(idx, value.to_owned());
            Ok(value)
        }));
        // list.insert(<list>, <value>, <idx>)
        module.set_item("insert", NativeFunction::new(|runtime, _, _| {
            let idx = runtime.get_local(-1)?
                .to_owned()
                .take_int()? as usize;
            let value = runtime.get_local(-2)?
                .to_owned();
            let list = runtime.get_local(-3)?
                .to_owned()
                .take_list()?;
            let len = list.borrow().len();
            if idx == len {
                list.borrow_mut().resize(len + 1, value.to_owned());
            } else if (idx as isize) < 0 || len - 1 < idx {
                Err(RuntimeError::IndexOutOfBounds { index: idx, size: len })?;
            } else {
                list.borrow_mut().insert(idx, value.to_owned());
            }
            Ok(value)
        }));
        // list.remove(<list>, <idx>)
        module.set_item("remove", NativeFunction::new(|runtime, _, _| {
            let idx = runtime.get_local(-1)?
                .to_owned()
                .take_int()? as usize;
            let list = runtime.get_local(-2)?
                .to_owned()
                .take_list()?;
            if (idx as isize) < 0 || list.borrow().len() - 1 < idx {
                Err(RuntimeError::IndexOutOfBounds { index: idx, size: list.borrow().len() })?;
            }

            Ok(list.borrow_mut().remove(idx).into())
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(ListSystem::NAME);
    ListSystem::declare(&mut module);

    module
}
