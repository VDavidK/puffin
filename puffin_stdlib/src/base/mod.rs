use puffin_runtime::runtime::Runtime;

mod array;
mod dictionary;
mod elements;
mod debug;

// use array::define_array_class;
// use dictionary::define_dictionary_class;
use crate::base::debug::{define_print_function, define_exit_function};
use crate::base::elements::text::define_text_element;
use crate::base::elements::flow::{define_flow_elements};

pub fn define(runtime: &mut Runtime) {
    // define_array_class(runtime);
    // define_dictionary_class(runtime);
    define_print_function(runtime);
    define_exit_function(runtime);
    define_text_element(runtime);
    define_flow_elements(runtime);
}