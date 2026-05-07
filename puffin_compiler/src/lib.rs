use std::path::PathBuf;
use puffin_ast::Ast;
use puffin_runtime::Chunk;
use crate::compiler::{CompileError, Compiler};

pub mod compiler;
mod scope;

pub fn compile_ast(name: impl AsRef<str>, ast: &Ast) -> Result<(Chunk, Vec<PathBuf>), CompileError> {
    let mut chunk = Chunk::new(name);
    let mut compiler = Compiler::new(&mut chunk);
    compiler.compile(ast)?;
    let deps = compiler.get_dependencies();

    Ok((chunk, deps))
}
