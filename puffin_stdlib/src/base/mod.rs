use puffin_runtime::runtime::Runtime;

mod array;
mod dictionary;
mod elements;
mod debug;

use array::define_array_class;
use dictionary::define_dictionary_class;
use crate::base::debug::define_print_function;
use crate::base::elements::text::define_text_element;
use crate::base::elements::hbox::define_hbox_element;

pub fn define(runtime: &mut Runtime) {
    define_array_class(runtime);
    define_dictionary_class(runtime);
    define_print_function(runtime);
    define_text_element(runtime);
    define_hbox_element(runtime);
}