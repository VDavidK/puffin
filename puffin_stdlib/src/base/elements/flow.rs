use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{NativeFunction, LayoutNode, LayoutDirection, Node, new_class, Value, ComponentNode, NodeType};
use crate::base::elements::{get_inner, get_inner_as, get_inner_nodes, get_props};

fn construct_flow(runtime: &mut Runtime, direction: LayoutDirection) -> Result<LayoutNode, RuntimeError> {
    let props = get_props(runtime)?;
    let child_elements = get_inner(runtime)?;
    let segments = if let Some(prop) = props.borrow().get(&"segments".into()) {
        prop.to_owned()
    } else {
        Value::Null
    };
    let node = LayoutNode {
        direction,
        node: child_elements,
        segments,
    };
    Ok(node)
}

pub fn define_flow_elements(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let hbox_class = new_class("HBox");
    let vbox_class = new_class("VBox");

    hbox_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "hbox".to_owned() })?;
        let children = construct_flow(runtime, LayoutDirection::Horizontal)?;
        this.borrow_mut().set_field("<children>", Node::Layout(children))?;
        Ok(Value::Null)
    }));

    hbox_class.borrow_mut().set_method("<construct>", NativeFunction::new(|_, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "hbox".to_owned() })?;

        Ok(this.borrow()
            .get_field("<children>")
            .expect("Hbox initialized without <children> property")
            .to_owned()
        )
    }));

    vbox_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "vbox".to_owned() })?;
        let children = construct_flow(runtime, LayoutDirection::Vertical)?;
        this.borrow_mut().set_field("<children>", Node::Layout(children))?;
        Ok(Value::Null)
    }));

    vbox_class.borrow_mut().set_method("<construct>", NativeFunction::new(|_, _, this| {
        let this = this.ok_or(RuntimeError::MissingThisInMethodCall{ name: "vbox".to_owned() })?;

        Ok(this.borrow()
            .get_field("<children>")
            .expect("Vbox initialized without <children> property")
            .to_owned()
        )
    }));

    runtime.add_global("hbox", hbox_class)?;
    runtime.add_global("vbox", vbox_class)?;
    Ok(())
}
