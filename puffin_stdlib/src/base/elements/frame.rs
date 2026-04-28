use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{new_class, ComponentNode, FrameNode, LayoutDirection, LayoutNode, NativeFunction, Node, TextNode, Value};

pub fn define_frame_element(runtime: &mut Runtime) {
    let frame_class = new_class("Frame");

    frame_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, argc, this| {
        let this = this.expect(format!("Constructor called for {} without this", "vbox").as_str());

        let child_elements = runtime
            .get_local(-1)?
            .to_owned()
            .take_list()?;
        let child_elements = child_elements.borrow()
            .iter()
            .map(|x| x.to_owned().take_instance())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|instance| ComponentNode { instance }.into())
            .collect::<Vec<_>>();

        let node = FrameNode {
            nodes: child_elements,
        };
        this.borrow_mut().set_field("<children>", Node::Frame(node));
        Ok(Value::Null)
    }));

    frame_class.borrow_mut().set_method("<layout>", NativeFunction::new(|_, _, this| {
        let this = this.expect(format!("Constructor called for {} without this", "frame").as_str());

        Ok(this.borrow()
            .get_field("<children>")
            .expect("Vbox initialized without <children> property")
            .to_owned()
        )
    }));

    runtime.add_global("frame", frame_class);
}
