use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, NativeFunction, Node, TextNode, Value};

pub fn define_text_element(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let text_class = new_class("Text");

    text_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let text = runtime.get_local(-1)?
            .to_owned();

        this.expect("Constructor called without instance")
            .borrow_mut()
            .set_field("text", text)?;

        Ok(Value::Null)
    }));

    text_class.borrow_mut().set_method("<layout>", NativeFunction::new(|_runtime, _argc, this| {
        let this = this.expect("How did you do this?");

        let text = this.borrow();
        let text = text
            .get_field("text")
            .expect("How did you do this?");

        let node = TextNode {
            content: text.to_string(),
        };

        Ok(Node::Text(node).into())
    }));

    runtime.add_global("text", text_class)?;
    Ok(())
}
