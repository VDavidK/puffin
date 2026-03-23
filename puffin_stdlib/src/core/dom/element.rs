use puffin_runtime::value::{Value, new_class, Module};
use crate::Declaration;

pub struct TextElement {

}

impl TextElement {

}

impl Declaration for TextElement {
    const NAME: &'static str = "TextElement";

    fn declare(module: &mut Module) {
        module.set_item("TextElement", Value::Class(new_class(Self::NAME)));
    }
}
