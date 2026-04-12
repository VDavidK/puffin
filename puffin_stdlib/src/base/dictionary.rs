use std::collections::HashMap;
use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{Class, NativeFunction, NativeValue, NativeValueTrait, Value};
use crate::base::array::NativeVector;

#[derive(Debug)]
struct NativeDictionary(HashMap<Value, Value>);

impl std::fmt::Display for NativeDictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeDictionary")
    }
}

impl NativeValueTrait for NativeDictionary {}

/// Dictionary defines the following methods:
/// - get(key_name) > Returns the value of the provided key in the dictionary or `null` if the key is not present.
/// - set(key_name, value) > Sets the provided key to the provided value.
/// - len() > Returns the number of elements in the dictionary.
/// - erase(key_name) > Erases the provided key from the dictionary, returning its value. Does nothing if the key is not present.
/// - has(key_name) > Returns a boolean indicating whether a key is present in the dictionary
/// - keys() > Returns an array of the dictionary's keys
/// - values() > Returns an array of the dictionary's values
pub fn define_dictionary_class(runtime: &mut Runtime) {
    let mut cls = Class::new("Dictionary");
    cls.set_field("dict", NativeValue::from(NativeDictionary(HashMap::new())));

    let get = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let key = runtime.get_local(-1)?.clone();
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        let val = dict.0.get(&key)
            .cloned()
            .unwrap_or(Value::Null);
        Ok(val)
    }, 2);

    let set = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-3)?.clone().take_instance()?;
        let key = runtime.get_local(-2)?.clone();
        let val = runtime.get_local(-1)?.clone();
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        dict.0.insert(key, val);
        Ok(Value::Null)
    }, 3);

    let len = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let arr = instance.borrow_mut()
            .get_field("arr")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let len = arr.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter")
            .0
            .len()
            .into();
        Ok(len)
    }, 1);

    let erase = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let key = runtime.get_local(-1)?.clone();
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let mut dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        dict.0.remove(&key);
        Ok(Value::Null)
    }, 2);

    let has = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-2)?.clone().take_instance()?;
        let key = runtime.get_local(-1)?.clone();
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        Ok(Value::Bool(dict.0.contains_key(&key)))
    }, 2);

    let keys = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        let val = NativeValue::new(NativeVector(dict.0.keys().into_iter().cloned().collect::<Vec<Value>>()));
        Ok(val.into())
    }, 1);

    let values = NativeFunction::new(|runtime| {
        let instance = runtime.get_local(-1)?.clone().take_instance()?;
        let dict = instance.borrow_mut()
            .get_field("dict")
            .expect("Called without self parameter")
            .clone()
            .take_native_value()?;
        let dict = dict.get_mut::<NativeDictionary>()
            .expect("Invalid self parameter");
        let val = NativeValue::new(NativeVector(dict.0.values().into_iter().cloned().collect::<Vec<Value>>()));
        Ok(val.into())
    }, 1);

    cls.set_method("get", get);
    cls.set_method("set", set);
    cls.set_method("len", len);
    cls.set_method("erase", erase);
    cls.set_method("has", has);
    cls.set_method("keys", keys);
    cls.set_method("values", values);

    runtime.add_global("Dictionary", cls);
}