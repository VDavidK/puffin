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

// /// Array defines the following methods:
// /// - get(idx) > Returns an element at the provided index of the Array. Returns `null` if out of bounds
// /// - push > Pushes an element to the back of the Array
// /// - pop > Removes the last element of the Array and returns it
// /// - resize > Modifies the capacity of the Array, removing excess elements or adding default values
// /// - push_front > Pushes an element to the front of the Array
// /// - pop_front > Removes the first element of the Array and returns it
// /// - len > Returns the number of elements in the Array
// /// - size > Returns the total capacity of the Array
// pub fn define_array_class(runtime: &mut Runtime) {
//     let mut cls = Class::new("Array");
//     cls.set_field("arr", NativeValue::from(NativeVector(vec![])));
//
//     let new = NativeFunction::new(|runtime, argc, this| {
//         let instance = runtime.get_local(-1)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         for i in -1 - (argc as isize)..-1 {
//             let arg = runtime.get_local(i as LocalOffset)?;
//             arr.0.push(arg.clone());
//         }
//         Ok(Value::Null)
//     });
//
//     let get = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-2)?.clone().take_instance()?;
//         let idx = runtime.get_local(-1)?.clone().take_int()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         let val = arr.0.get(idx as usize)
//             .cloned()
//             .unwrap_or(Value::Null);
//         Ok(val)
//     });
//
//     let push = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-2)?.clone().take_instance()?;
//         let value = runtime.get_local(-1)?.clone();
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         arr.0.push(value);
//         Ok(Value::Null)
//     });
//
//     let pop = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-1)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         let val = arr.0.pop().unwrap_or(Value::Null);
//         Ok(val)
//     });
//
//     let resize = NativeFunction::new(|runtime, argc| {
//         let size = runtime.get_local(-1)?.clone().take_int()?;
//         let instance = runtime.get_local(-2)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         arr.0.resize(size as usize, Value::Null);
//         Ok(Value::Null)
//     });
//
//     let push_front = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-2)?.clone().take_instance()?;
//         let value = runtime.get_local(-1)?.clone();
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         arr.0.insert(0, value);
//         Ok(Value::Null)
//     });
//
//     let pop_front = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-1)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let mut arr = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter");
//         if arr.0.len() == 0 {
//            Ok(Value::Null)
//         } else {
//             let value = arr.0.remove(0);
//             Ok(value)
//         }
//     });
//     cls.set_method("pop_front", pop_front);
//
//     let len = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-1)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let len = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter")
//             .0
//             .len()
//             .into();
//         Ok(len)
//     });
//
//     let size = NativeFunction::new(|runtime, argc| {
//         let instance = runtime.get_local(-1)?.clone().take_instance()?;
//         let arr = instance.borrow_mut()
//             .get_field("arr")
//             .expect("Called without self parameter")
//             .clone()
//             .take_native_value()?;
//         let capacity = arr.get_mut::<NativeVector>()
//             .expect("Invalid self parameter")
//             .0
//             .capacity()
//             .into();
//         Ok(capacity)
//     });
//
//     cls.set_constructor(new);
//     cls.set_method("get", get);
//     cls.set_method("push", push);
//     cls.set_method("pop", pop);
//     cls.set_method("resize", resize);
//     cls.set_method("push_front", push_front);
//     cls.set_method("len", len);
//     cls.set_method("size", size);
//
//     runtime.add_global("Array", cls);
// }