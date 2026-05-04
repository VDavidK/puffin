use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, ComponentNode, FrameNode, NativeFunction, Node, Value};

pub fn define_frame_element(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let frame_class = new_class("Frame");

    frame_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "vbox".to_owned() })?;

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
        this.borrow_mut().set_field("<children>", Node::Frame(node))?;
        Ok(Value::Null)
    }));

    frame_class.borrow_mut().set_method("<layout>", NativeFunction::new(|_, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "frame".to_owned() })?;

        Ok(this.borrow()
            .get_field("<children>")
            .expect("Vbox initialized without <children> property")
            .to_owned()
        )
    }));

    runtime.add_global("frame", frame_class)?;
    Ok(())
}
