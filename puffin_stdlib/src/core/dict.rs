use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;

struct DictSystem;

impl Declaration for DictSystem {
    const NAME: &'static str = "dict";

    fn declare(module: &mut Module) {
        module.set_item("insert", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-3)?
                .to_owned()
                .eval()?
                .take_dictionary()?;
            let key = runtime.get_local(-2)?.eval()?;
            let value = runtime.get_local(-1)?.eval()?;

            dict.borrow_mut().insert(key.to_owned(), value.to_owned());
            Ok(value)
        }));
        module.set_item("contains", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_dictionary()?;

            let key = runtime.get_local(-1)?.eval()?;
            Ok(dict.borrow_mut().contains_key(&key).into())
        }));
        module.set_item("remove", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-2)?
                .to_owned()
                .eval()?
                .take_dictionary()?;
            let key = runtime.get_local(-1)?.eval()?;

            Ok(dict.borrow_mut().remove(&key).unwrap_or(Value::Null))
        }));
        module.set_item("keys", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_dictionary()?;
            let list = dict.borrow()
                .keys()
                .cloned()
                .collect::<Vec<Value>>()
                .into();
            Ok(list)
        }));
        module.set_item("values", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_dictionary()?;
            let list = dict.borrow()
                .values()
                .cloned()
                .collect::<Vec<Value>>()
                .into();
            Ok(list)
        }));
        module.set_item("entries", NativeFunction::new(|runtime, _, _| {
            let dict = runtime.get_local(-1)?
                .to_owned()
                .eval()?
                .take_dictionary()?;
            let list = dict.borrow()
                .iter()
                .map(|(k, v)| vec![k.to_owned(), v.to_owned()].into())
                .collect::<Vec<Value>>()
                .into();
            Ok(list)
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(DictSystem::NAME);
    DictSystem::declare(&mut module);

    module
}
