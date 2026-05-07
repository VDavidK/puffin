use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, ComponentNode, FrameNode, NativeFunction, Node, NodeType, Value};

pub fn define_frame_element(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    let frame_class = new_class("Frame");

    frame_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "frame".to_owned() })?;

        let child_elements = runtime
            .get_local(-2)?
            .to_owned()
            .take_list()?;
        let child_elements = child_elements.borrow()
            .iter()
            .map(|x| x.to_owned().take_node())
            .collect::<Result<Vec<_>, _>>()?;

        let node = FrameNode {
            nodes: child_elements,
        };
        this.borrow_mut().set_field("<children>", Node::Frame(node))?;
        Ok(Value::Null)
    }));

    frame_class.borrow_mut().set_method("<construct>", NativeFunction::new(|_, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "frame".to_owned() })?;

        Ok(this.borrow()
            .get_field("<children>")
            .expect("Frame initialized without <children> property")
            .to_owned()
        )
    }));

    runtime.add_global("frame", frame_class)?;
    Ok(())
}
