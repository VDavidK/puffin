use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, NativeFunction, Node, StringType, TextNode, Value};

pub fn define_text_element(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let text_class = new_class("Text");

    text_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let this = this.expect("Constructor called without instance");
        let props = runtime.get_local(-1)?
            .to_owned()
            .take_dictionary()?;
        let text = runtime.get_local(-2)?
            .to_owned();

        this.borrow_mut()
            .set_field("text", text)?;
        this.borrow_mut()
            .set_field("<color>", props.borrow()
                .get(&"color".to_string().into())
                .unwrap_or(&"reset".into())
                .to_owned()
            )?;

        this.borrow_mut()
            .set_field("<bg>", props.borrow()
                .get(&"bg".to_string().into())
                .unwrap_or(&"reset".into())
                .to_owned()
            )?;

        Ok(Value::Null)
    }));

    text_class.borrow_mut().set_method("<layout>", NativeFunction::new(|_runtime, _argc, this| {
        let this = this.expect("How did you do this?");

        let this = this.borrow();
        let text = this
            .get_field("text")
            .expect("How did you do this?");

        let fg_color = this
            .get_field("<color>").expect("How did you do this?")
            .to_owned();
        let bg_color = this
            .get_field("<bg>").expect("How did you do this?")
            .to_owned();
        let node = TextNode {
            content: text.to_owned(),
            text_color: fg_color,
            bg_color,
        };

        Ok(Node::Text(node).into())
    }));

    runtime.add_global("text", text_class)?;
    Ok(())
}
