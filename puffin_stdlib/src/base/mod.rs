use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use crate::base::casting::{define_bool_cast_fn, define_float_cast_fn, define_int_cast_fn, define_string_cast_fn};

mod elements;
mod debug;
mod casting;
mod utils;

// use array::define_array_class;
// use dictionary::define_dictionary_class;
use crate::base::debug::{define_exit_function, define_print_function};
use crate::base::elements::text::define_text_element;
use crate::base::elements::image::define_image_element;
use crate::base::elements::flow::define_flow_elements;
use crate::base::elements::frame::define_frame_element;
use crate::base::elements::input::define_input_element;
use crate::base::utils::{define_block_fn, define_clone_fn, define_len_fn, define_render_fn};

pub fn define(runtime: &mut Runtime) -> Result<(), RuntimeError>  {
    // define_array_class(runtime);
    // define_dictionary_class(runtime);
    define_print_function(runtime)?;
    define_exit_function(runtime)?;
    define_text_element(runtime)?;
    define_flow_elements(runtime)?;
    define_frame_element(runtime)?;
    define_input_element(runtime)?;
    define_image_element(runtime)?;
    define_len_fn(runtime)?;
    define_block_fn(runtime)?;
    define_render_fn(runtime)?;
    define_clone_fn(runtime)?;
    define_string_cast_fn(runtime)?;
    define_int_cast_fn(runtime)?;
    define_float_cast_fn(runtime)?;
    define_bool_cast_fn(runtime)?;
    Ok(())
}