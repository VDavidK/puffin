pub mod position;
pub mod span;
mod snippet;
pub mod expression;
pub mod token;
pub mod statement;
pub mod declaration;

use declaration::{Declaration};

#[derive(Debug)]
pub enum VarType {
    Let,
    Const,
}

#[derive(Debug)]
pub struct Ast {
    pub declarations: Vec<Declaration>,
}

impl Ast {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self {
            declarations,
        }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}