use std::path::PathBuf;
use puffin_ast::Ast;
use puffin_runtime::Chunk;
use crate::codegen::{GenError, CodeGenerator};

pub mod codegen;
mod scope;

pub fn generate_from_ast(name: impl AsRef<str>, ast: &Ast) -> Result<(Chunk, Vec<PathBuf>), GenError> {
    let mut chunk = Chunk::new(name);
    let mut code_generator = CodeGenerator::new(&mut chunk);
    code_generator.generate(ast)?;
    let deps = code_generator.get_dependencies();

    Ok((chunk, deps))
}
