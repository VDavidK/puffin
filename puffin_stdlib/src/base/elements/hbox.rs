use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{NativeFunction, LayoutNode, LayoutDirection, Node};

pub fn define_hbox_element(runtime: &mut Runtime) {
    runtime.add_global("hbox", NativeFunction::new(|runtime, _argc| {
        let child_elements = runtime
            .get_local(-1)?
            .to_owned()
            .take_list()?
            .into_iter()
            .map(|x| x.take_node())
            .collect::<Result<Vec<_>, _>>()?;

        let layout = LayoutNode {
            direction: LayoutDirection::Horizontal,
            nodes: child_elements,
        };

        Ok(Node::Layout(layout).into())
    }));
}
