use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;

struct FileSystem;

impl Declaration for FileSystem {
    const NAME: &'static str = "fs";

    fn declare(module: &mut Module) {
        module.set_item("write", NativeFunction::new(|runtime| {
            let path = runtime.get_local(-2)?
                .to_owned()
                .take_string()?;

            let content = runtime.get_local(-1)?
                .to_string();

            std::fs::write(path, content)?;

            Ok(Value::Null)
        }, 2))
    }
}

pub fn module() -> Module {
    let mut module = Module::new(FileSystem::NAME);
    FileSystem::declare(&mut module);

    module
}
