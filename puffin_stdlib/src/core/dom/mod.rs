use puffin_runtime::value::Module;
use crate::declaration::Declaration;

pub mod element;

pub fn create_module() -> Module {
    let mut module = Module::new("dom");
    element::TextElement::declare(&mut module);

    module
}