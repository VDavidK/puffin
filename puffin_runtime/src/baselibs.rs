use crate::{runtime::Runtime, value::{NativeFunction, Value}, RuntimeError};
use crate::ui::TextNode;
use crate::value::{Class, NativeValue, NativeValueTrait};
#[derive(Debug)]
struct NativeVector(Vec<Value>);

impl std::fmt::Display for NativeVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeVector")
    }
}

impl NativeValueTrait for NativeVector {}

pub fn define_print_function(runtime: &mut Runtime) {
    runtime.add_global("print", NativeFunction::new(|runtime| {
        // Get value
        let value = runtime.get_local(-1)?;

        // Render value
        // TODO: Fix
        println!("{}", value);

        // Wait for user input
        runtime.poll()?;

        // Return null
        Ok(Value::Null)
    }, 1));
}

pub fn define_text_element(runtime: &mut Runtime) {
    runtime.add_global("text", NativeFunction::new(|runtime| {
        let text = runtime.get_local(-1)?;

        runtime.push_node(TextNode {
            content: text.to_string(),
        });

        Ok(Value::Null)
    }, 1));
}

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
    }, 0);
    cls.set_method("pop", pop);
    runtime.add_global("Array", cls);
}