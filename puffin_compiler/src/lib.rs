use puffin_ast::Ast;
use puffin_runtime::Chunk;

pub mod ir;
mod compiler;

pub fn compile_ast(name: impl AsRef<str>, ast: Ast) -> Chunk {
    let mut chunk = Chunk::new(name);

    todo!()
}
