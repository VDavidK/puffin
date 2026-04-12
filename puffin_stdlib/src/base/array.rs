use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{Class, NativeFunction, NativeValue, NativeValueTrait, Value};

#[derive(Debug)]
pub(crate) struct NativeVector(pub Vec<Value>);

impl std::fmt::Display for NativeVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeVector")
    }
}

impl NativeValueTrait for NativeVector {}

/// Array defines the following methods:
/// - get(idx) > Returns an element at the provided index of the Array. Returns `null` if out of bounds
/// - push > Pushes an element to the back of the Array
/// - pop > Removes the last element of the Array and returns it
/// - resize > Modifies the capacity of the Array, removing excess elements or adding default values
/// - push_front > Pushes an element to the front of the Array
/// - pop_front > Removes the first element of the Array and returns it
/// - len > Returns the number of elements in the Array
/// - size > Returns the total capacity of the Array
pub fn define_array_class(runtime: &mut Runtime) {
    let mut cls = Class::new("Array");
    cls.set_field("arr", NativeValue::from(NativeVector(vec![])));

    let get = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let idx = runtime.get_local(-1)?.clone().take_int()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        let val = arr.0.get(idx as usize)
            .cloned()
            .unwrap_or(Value::Null);
        Ok(val)
    }, 2);

    let push = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let value = runtime.get_local(-1)?.clone();
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        arr.0.push(value);
        Ok(Value::Null)
    }, 2);

    let pop = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        let val = arr.0.pop().unwrap_or(Value::Null);
        Ok(val)
    }, 1);
    let resize = NativeFunction::new(|runtime| {
        let size = runtime.get_local(-1)?.clone().take_int()?;
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        arr.0.resize(size as usize, Value::Null);
        Ok(Value::Null)
    }, 2);

    let push_front = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let value = runtime.get_local(-1)?.clone();
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        arr.0.insert(0, value);
        Ok(Value::Null)
    }, 2);

    let pop_front = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut arr = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter");
        if arr.0.len() == 0 {
           Ok(Value::Null)
        } else {
            let value = arr.0.remove(0);
            Ok(value)
        }
    }, 1);
    cls.set_method("pop_front", pop_front);

    let len = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let len = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter")
            .0
            .len()
            .into();
        Ok(len)
    }, 1);

    let size = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let capacity = arr.get_mut::<NativeVector>()
            .expect("Invalid self parameter")
            .0
            .capacity()
            .into();
        Ok(capacity)
    }, 1);

    cls.set_method("get", get);
    cls.set_method("push", push);
    cls.set_method("pop", pop);
    cls.set_method("resize", resize);
    cls.set_method("push_front", push_front);
    cls.set_method("len", len);
    cls.set_method("size", size);

    runtime.add_global("Array", cls);
}