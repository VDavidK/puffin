use std::collections::HashMap;
use puffin_runtime::value::{Module, NativeFunction, Value};
use crate::declaration::Declaration;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::prelude::CommandExt;
use puffin_runtime::RuntimeError;
use std::process::Command;
use puffin_runtime::chunk::LocalOffset;

struct ShellSystem;

impl Declaration for ShellSystem {
    const NAME: &'static str = "shell";

    fn declare(module: &mut Module) {
        module.set_item("exec", NativeFunction::new(|runtime, argc, _| {
            let command = runtime.get_local(-(argc as LocalOffset))?
                .to_owned()
                .take_string()?;
            let command = command
                .borrow();

            let args = runtime.get_locals(-(argc as LocalOffset) + 1, 0)?
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>();

            let mut cmd = Command::new(&*command);
            cmd.args(args);
            let _status = cmd.status().expect(format!("Failed to execute command {}", command.as_str()).as_str());
            let mut res = HashMap::new();
            let out = cmd.output()?;
            res.insert(Value::from("stdout"), Value::from(String::from_utf8_lossy(&out.stdout).to_string()));
            res.insert(Value::from("stderr"), Value::from(String::from_utf8_lossy(&out.stderr).to_string()));
            Ok(Value::from(res))
        }));
    }
}

pub fn module() -> Module {
    let mut module = Module::new(ShellSystem::NAME);
    ShellSystem::declare(&mut module);

    module
}
