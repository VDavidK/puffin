use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, NativeFunction, Node, TextNode, Value};
use crate::base::elements::EVENT_HANDLER_ONKEY;

pub fn define_input_element(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let class = new_class("Input");

    class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let text = runtime.get_local(-1)?
            .to_owned();

        this.expect("Constructor called without instance")
            .borrow_mut()
            .set_field("text", text)?;

        Ok(Value::Null)
    }));

    class.borrow_mut().set_method("<layout>", NativeFunction::new(|_runtime, _argc, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "input".to_owned() })?;

        let text = this.borrow();
        let text = text
            .get_field("text")
            .expect("How did you do this?")
            .to_owned();
        let node = TextNode {
            content: text,
            text_color: Value::Null,
            bg_color: Value::Null,
        };

        Ok(Node::Text(node).into())
    }));

    class.borrow_mut().set_handler(EVENT_HANDLER_ONKEY, NativeFunction::new(|runtime, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "input".to_owned() })?;
        let key = runtime.get_local(-1)?
            .to_owned();
        let char = key.take_string()?
            .borrow()
            .chars()
            .nth(0)
            .expect("");
        let current = this.borrow()
            .get_field("text")
            .expect("text property undefined for input element")
            .to_owned()
            .take_string()?;
        current.borrow_mut().push(char);
        Ok(Value::Null)
    }));

    runtime.add_global("input", class)?;
    Ok(())
}
