use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;
use std::fs::OpenOptions;
use std::io::Write;
use puffin_runtime::chunk::LocalOffset;

struct StringSystem;

impl Declaration for StringSystem {
    const NAME: &'static str = "string";

    fn declare(module: &mut Module) {
        // string.push(<str>, <value>)
        module.set_item("push", NativeFunction::new(|runtime, _, _| {
            let value = runtime.get_local(-1)?.eval()?.to_string();
            let string = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_string()?;

            string.borrow_mut().push_str(value.to_owned().as_str());
            Ok(value.into())
        }));
        // string.push_front(<str>, <value>)
        module.set_item("push_front", NativeFunction::new(|runtime, _, _| {
            let value = runtime.get_local(-1)?.eval()?.to_string();
            let string = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_string()?;

            string.borrow_mut().insert_str(0, value.to_owned().as_str());
            Ok(value.into())
        }));
        // string.pop(<str>)
        module.set_item("pop", NativeFunction::new(|runtime, _, _| {
            let string = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_string()?;
            Ok(string.borrow_mut().pop().map_or(Value::Null, |v| v.into()))
        }));
        // string.pop_front(<str>)
        module.set_item("pop_front", NativeFunction::new(|runtime, _, _| {
            let string = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_string()?;
            if string.borrow().is_empty() {
                Ok(Value::Null)
            } else {
                Ok(string.borrow_mut().remove(0).into())
            }
        }));
        // string.replace(<str>, <from>, <to>)
        module.set_item("replace", NativeFunction::new(|runtime, _, _| {
            let to = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .to_string();
            let from = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_string()?;
            let string = runtime.get_local(-3)?
                .to_owned()
                .eval()?
                .take_string()?;
            Ok(string.borrow().replace(from.borrow().as_str(), to.as_str()).into())
        }));
        // string.insert(<str>, <value>, <idx>)
        module.set_item("insert", NativeFunction::new(|runtime, _, _| {
            let idx = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_int()? as usize;
            let to_insert = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .to_string();
            let string = runtime.get_local(-3)?
                .to_owned()
                .eval()?
                .take_string()?;
            if idx > string.borrow().len() {
                return Ok(Value::Null);
            }
            string.borrow_mut().insert_str(idx, to_insert.as_str());
            Ok(to_insert.into())
        }));
        // string.remove(<str>, <idx>)
        module.set_item("remove", NativeFunction::new(|runtime, _, _| {
            let idx = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_int()? as usize;
            let string = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_string()?;

            if string.borrow().is_empty() {
                return Ok(Value::Null);
            }
            Ok(string.borrow_mut().remove(idx).into())
        }));
        // string.contains(<str>, <pattern>)
        module.set_item("contains", NativeFunction::new(|runtime, _, _| {
            let pattern = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_string()?;
            let string = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_string()?;
            Ok(string.borrow().contains(pattern.borrow().as_str()).into())
        }));
        // string.to_lower(<str>)
        module.set_item("to_lower", NativeFunction::new(|runtime, _, _| {
            let string = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_string()?;
            Ok(string.borrow_mut().to_lowercase().into())
        }));
        module.set_item("to_upper", NativeFunction::new(|runtime, _, _| {
            let string = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_string()?;
            Ok(string.borrow_mut().to_uppercase().into())
        }));
        // string.split(<str>, <delim>, <n>?)
        module.set_item("split", NativeFunction::new(|runtime, argc, _| {
            let arg_start = -(argc as LocalOffset);
            let string = runtime.get_local(arg_start)?
                .to_owned()
                .eval()?
                .take_string()?;
            let delim = runtime.get_local(arg_start + 1)?
                .to_owned()
                .eval()?
                .take_string()?;
            let res = if argc > 2 {
                let max_splits = runtime.get_local(arg_start + 2)?
                    .to_owned()
                    .eval()?
                    .take_int()? as usize;
                string.borrow_mut()
                    .splitn(max_splits, delim.borrow().as_str())
                    .map(String::from)
                    .map(Value::from)
                    .collect::<Vec<Value>>()
            } else {
                string.borrow_mut()
                    .split(delim.borrow().as_str())
                    .map(String::from)
                    .map(Value::from)
                    .collect::<Vec<Value>>()
            };

            Ok(Value::from(res))
        }));
        // string.substring(<str>, <idx>, <len>)
        module.set_item("substring", NativeFunction::new(|runtime, _, _| {
            let string = runtime.get_local(-3)?
                .to_owned()
                .eval()?
                .take_string()?;
            let idx = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_int()?;
            let len = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_int()?;
            if idx < 0 {
                Err(RuntimeError::UnexpectedNegativeNumber { var_name: "string.substring(.., idx, ..)".to_string(), num: idx as isize, })?
            }
            if len < 0 {
                Err(RuntimeError::UnexpectedNegativeNumber { var_name: "string.substring(.., .., len)".to_string(), num: idx as isize, })?
            }
            if idx as usize + len as usize >= string.borrow().len() {
                Err(RuntimeError::IndexOutOfBounds { index: idx as usize, size: string.borrow().len() })?
            }
            let slice = string.borrow();
            let slice = slice.as_str();
            Ok(slice[(idx as usize)..(idx+len) as usize].to_string().into())
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(StringSystem::NAME);
    StringSystem::declare(&mut module);

    module
}
