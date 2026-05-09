use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{NativeFunction, Value};

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
