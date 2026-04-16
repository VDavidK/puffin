use crate::{runtime::Runtime, value::{NativeFunction, Value}, RuntimeError};
use crate::ui::TextNode;

pub fn define_print_function(runtime: &mut Runtime) {
    runtime.add_global("print", NativeFunction::new(|runtime, argc| {
        // Get value
        let value = runtime.get_local(-1)?;

        // Render value
        // TODO: Fix
        println!("{}", value);

        // Wait for user input
        runtime.poll()?;

        // Return null
        Ok(Value::Null)
    }));
}

pub fn define_text_element(runtime: &mut Runtime) {
    runtime.add_global("text", NativeFunction::new(|runtime, argc| {
        let text = runtime.get_local(-1)?;

        runtime.push_node(TextNode {
            content: text.to_string(),
        });

        Ok(Value::Null)
    }));
}