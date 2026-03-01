use puffin_ast::Ast;
use puffin_runtime::Chunk;
use crate::compiler::{CompileError, Compiler};

pub mod compiler;

pub fn compile_ast(name: impl AsRef<str>, ast: &Ast) -> Result<Chunk, CompileError> {
    let mut chunk = Chunk::new(name);
    let mut compiler = Compiler::new(&mut chunk);
    compiler.compile(ast)?;

    Ok(chunk)
}
