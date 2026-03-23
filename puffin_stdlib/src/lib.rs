mod declaration;

pub mod core;
#[cfg(feature = "vendor")]
pub mod vendor;

pub(crate) use declaration::Declaration;
