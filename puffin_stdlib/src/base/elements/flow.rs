use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{NativeFunction, LayoutNode, LayoutDirection, Node};

fn construct_flow(runtime: &mut Runtime, direction: LayoutDirection) -> Result<LayoutNode, RuntimeError> {
    let child_elements = runtime
        .get_local(-1)?
        .to_owned()
        .take_list()?;
    let child_elements = child_elements.borrow()
        .iter()
        .map(|x| x.to_owned().take_node())
        .collect::<Result<Vec<_>, _>>()?;

    let node = LayoutNode {
        direction,
        nodes: child_elements,
    };
    Ok(node)
}

pub fn define_flow_elements(runtime: &mut Runtime) {
    runtime.add_global("hbox", NativeFunction::new(|runtime, _argc| {
        let node = construct_flow(runtime, LayoutDirection::Horizontal)?;
        Ok(Node::Layout(node).into())
    }));
    runtime.add_global("vbox", NativeFunction::new(|runtime, _argc| {
        let node = construct_flow(runtime, LayoutDirection::Vertical)?;
        Ok(Node::Layout(node).into())
    }));
}
