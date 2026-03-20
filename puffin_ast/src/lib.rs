#![allow(unused, dead_code)] // TODO: Remove this before going to prod (please)
pub mod position;
pub mod span;
mod snippet;
pub mod expression;
pub mod token;
pub mod statement;
pub mod declaration;
pub mod markup;

use declaration::{Declaration};

#[derive(Debug, PartialEq, Eq)]
pub enum VarType {
    Let,
    Const,
}

#[derive(Debug)]
pub struct Ast {
    pub component_name: String,
    pub declarations: Vec<Declaration>,
}

impl Ast {
    pub fn new(component_name: String) -> Self {
        Self {
            component_name,
            declarations: vec![],
        }
    }

    pub fn add_decl(&mut self, decl: Declaration) -> &mut Self {
        self.declarations.push(decl);
        self
    }

    pub fn dump(&self) {
        dbg!(self);
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}