use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{NativeFunction, LayoutNode, LayoutDirection, Node, new_class, Value, ComponentNode};

fn construct_flow(runtime: &mut Runtime, direction: LayoutDirection) -> Result<LayoutNode, RuntimeError> {
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

    let node = LayoutNode {
        direction,
        nodes: child_elements,
    };
    Ok(node)
}

pub fn define_flow_elements(runtime: &mut Runtime) {
    let hbox_class = new_class("HBox");

    hbox_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, argc, this| {
        let children = construct_flow(runtime, LayoutDirection::Horizontal)?;

        this.expect("How did you do this?")
            .borrow_mut()
            .set_field("children", Node::Layout(children));

        Ok(Value::Null)
    }));

    hbox_class.borrow_mut().set_method("<layout>", NativeFunction::new(|runtime, argc, this| {
        let this = this.expect("How did you do this?");

        Ok(this.borrow()
            .get_field("children")
            .expect("How did you do this?")
            .to_owned()
        )
    }));

    runtime.add_global("hbox", hbox_class);

    // runtime.add_global("hbox", NativeFunction::new(|runtime, _argc| {
    //     let node = construct_flow(runtime, LayoutDirection::Horizontal)?;
    //     Ok(Node::Layout(node).into())
    // }));
    // runtime.add_global("vbox", NativeFunction::new(|runtime, _argc| {
    //     let node = construct_flow(runtime, LayoutDirection::Vertical)?;
    //     Ok(Node::Layout(node).into())
    // }));
}
