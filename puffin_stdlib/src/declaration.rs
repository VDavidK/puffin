use puffin_runtime::value::Module;

pub trait Declaration {
    const NAME: &'static str;

    fn declare(module: &mut Module);
}
