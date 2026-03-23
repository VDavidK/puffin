
use puffin_runtime::vm::Vm;

mod declaration;

pub mod core;
#[cfg(feature = "vendor")]
pub mod vendor;

pub(crate) use declaration::Declaration;

pub fn open(vm: &mut Vm) {
    core::open(vm);

    #[cfg(feature = "vendor")]
    vendor::open(vm);
}
