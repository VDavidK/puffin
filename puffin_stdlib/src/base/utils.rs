use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{BlockNode, NativeFunction, Node, Value};

pub fn define_len_fn(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    let func = NativeFunction::new(|runtime, _argc, _| {
        let collection = runtime.get_local(-1)?.to_owned().eval()?;

        let len = match collection {
            Value::String(s) => s.borrow().len(),
            Value::List(l) => l.borrow().len(),
            Value::Dictionary(d) => d.borrow().len(),
            v => Err(RuntimeError::IncorrectType {ty: v.type_name().to_owned(), expected: "string, list or dictionary".to_owned()})?,
        };

        Ok(len.into())
    });

    runtime.add_global("len", func)?;
    Ok(())
}

pub fn define_block_fn(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    runtime.add_global("block", NativeFunction::new(|runtime, _, _| {
        let nodes = runtime
            .get_local(-1)?
            .to_owned()
            .take_list()?;

        let inner = nodes
            .borrow()
            .iter()
            .cloned()
            .map(|v| v.take_node())
            .collect::<Result<Vec<_>, _>>()?;

        let block = BlockNode {
            inner,
        };

        Ok(Node::Block(block).into())
    }))?;

    Ok(())
}

pub fn define_render_fn(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    runtime.add_global("render", NativeFunction::new(|runtime, _, _| {
        let instance = runtime.get_local(-1)?
            .to_owned()
            .take_instance()?;

        let node = instance.borrow()
            .get_field("<layout>")
            .cloned()
            .expect("<layout> should always exist");

        Ok(node)
    }))?;

    Ok(())
}

pub fn define_clone_fn(runtime: &mut Runtime) -> Result<(), RuntimeError> {
    runtime.add_global("clone", NativeFunction::new(|runtime, _, _| {
        Ok(runtime.get_local(-1)?.eval()?.deep_clone()?)
    }))?;

    Ok(())
}
