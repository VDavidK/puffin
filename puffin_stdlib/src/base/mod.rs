use puffin_runtime::runtime::Runtime;

mod array;
mod dictionary;

use array::define_array_class;
use dictionary::define_dictionary_class;

pub fn define(runtime: &mut Runtime) -> Result<(), ()> {
    define_array_class(runtime);
    define_dictionary_class(runtime);
    Ok(())
}