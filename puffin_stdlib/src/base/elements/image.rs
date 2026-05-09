use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{new_class, ImageNode, NativeFunction, Node, StringType, TextNode, Value};

pub fn define_image_element(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let image_class = new_class("Text");

    image_class.borrow_mut().set_constructor(NativeFunction::new(|runtime, _argc, this| {
        let this = this.expect("Constructor called without instance");
        let props = runtime.get_local(-1)?
            .to_owned()
            .take_dictionary()?;
        let path = runtime.get_local(-2)?
            .to_owned();

        this.borrow_mut()
            .set_field("path", path)?;

        Ok(Value::Null)
    }));

    image_class.borrow_mut().set_method("<construct>", NativeFunction::new(|_runtime, _argc, this| {
        let this = this.expect("How did you do this?");

        let this = this.borrow();
        let path = this
            .get_field("path")
            .expect("How did you do this?");

        let node = ImageNode {
            path: path.to_owned(),
        };

        Ok(Node::Image(node).into())
    }));

    runtime.add_global("image", image_class)?;
    Ok(())
}
