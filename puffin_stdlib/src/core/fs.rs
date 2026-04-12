use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;
use std::fs::OpenOptions;
use std::io::Write;

struct FileSystem;

impl Declaration for FileSystem {
    const NAME: &'static str = "fs";

    fn declare(module: &mut Module) {
        module.set_item("write", NativeFunction::new(|runtime| {
            let path = runtime.get_local(-3)?
                .to_owned()
                .take_string()?;

            let content = runtime.get_local(-2)?
                .to_string();

            let append = runtime.get_local(-1)
                .cloned()
                .unwrap_or(Value::Bool(false))
                .take_bool()?;

            let mut file = OpenOptions::new()
                .write(true)
                .append(append)
                .create(true)
                .open(path)
                .expect("Could not create file");

            if let Err(e) = writeln!(file, "{}", content) {

            }
            // std::fs::write(path, content)?;

            Ok(Value::Null)
        }, 3));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(FileSystem::NAME);
    FileSystem::declare(&mut module);

    module
}
