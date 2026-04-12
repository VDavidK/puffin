mod declaration;

pub mod core;
#[cfg(feature = "vendor")]
pub mod vendor;
pub mod base;

pub(crate) use declaration::Declaration;
