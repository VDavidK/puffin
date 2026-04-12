use puffin_runtime::runtime::Runtime;

mod array;

use array::define_array_class;

pub fn define(runtime: &mut Runtime) -> Result<(), ()> {
    define_array_class(runtime);
    Ok(())
}