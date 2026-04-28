use puffin_runtime::runtime::Runtime;
use puffin_runtime::value::{NativeFunction, Node, TextNode};

pub fn define_text_element(runtime: &mut Runtime) {
    runtime.add_global("text", NativeFunction::new(|runtime, argc| {
        let text = runtime.get_local(-1)?;

        let node = TextNode {
            content: text.to_string(),
        };

        Ok(Node::Text(node).into())
    }));
}
