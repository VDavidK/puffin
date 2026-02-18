use crate::token::{Token};
use crate::expression::{Expression};
use crate::statement::{Statement};
use crate::{VarType};

#[derive(Debug)]
pub struct ComponentDeclaration {
    pub name: Option<Token>,
    pub parameters: Vec<Token>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub name: Token,
    pub value: Box<Expression>,
    pub var_type: VarType,
}

#[derive(Debug)]
pub struct LayoutDeclaration {
    /* TODO: Figure out what layout members are exactly */
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct LambdaFunctionBinding {
    pub parameters: Vec<Token>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct DirectBindings {
    pub names: Vec<Token>,
}

#[derive(Debug)]
pub struct BindingDeclaration {
    pub name: Token,
    pub binding: LayoutItemProp,
}

#[derive(Debug)]
pub enum LayoutItemProp {
    DirectBindings(DirectBindings),
    Lambda(LambdaFunctionBinding),
}

#[derive(Debug)]
pub struct LayoutItemDeclaration {
    pub name: Token,
    pub bindings: Vec<BindingDeclaration>,
    pub string_literal: Option<Token>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct SignalDeclaration {
    pub name: Token,
    pub parameters: Vec<Token>,
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub decorator: Option<Decorator>,
    pub name: Token,
    pub parameters: Vec<Token>,
    pub block: Box<Statement>,
}

#[derive(Debug)]
pub struct RequireDeclaration {
    module_name: Token,
}

#[derive(Debug)]
pub struct UseDeclaration {
    name: Box<Expression>,
}

#[derive(Debug)]
pub struct ExportDeclaration {
    exported: Box<Declaration>,
}

#[derive(Debug)]
pub struct Decorator {
    pub name: Token,
    pub parameters: Vec<Token>,
}

#[derive(Debug)]
pub struct EnumDeclaration {
    pub name: Token,
    pub members: Vec<Token>,
}

#[derive(Debug)]
pub enum Declaration {
    Component(ComponentDeclaration),
    Var(VarDeclaration),
    Layout(LayoutDeclaration),
    LayoutItem(LayoutItemDeclaration),
    Signal(SignalDeclaration),
    Method(MethodDeclaration),
    Require(RequireDeclaration),
    Use(UseDeclaration),
    Export(ExportDeclaration),
    Enum(EnumDeclaration),
}

impl Decorator {
    pub fn new(name: Token, parameters: Vec<Token>) -> Self {
        Self {
            name,
            parameters,
        }
    }
}

impl RequireDeclaration {
    pub fn new(module_name: Token) -> Self {
        Self {
            module_name
        }
    }
}

impl UseDeclaration {
    pub fn new(name: Expression) -> Self {
        Self {
            name: Box::new(name),
        }
    }
}

impl ComponentDeclaration {
    pub fn new(parameters: Vec<Token>, declarations: Vec<Declaration>) -> Self {
        Self {
            name: None,
            parameters,
            declarations,
        }
    }

    pub fn with_name(mut self, name: Token) -> Self {
        self.name = Some(name);
        self
    }
}
impl VarDeclaration {
    pub fn new(name: Token, value: Expression, var_type: VarType) -> Self {
        Self {
            name,
            value: Box::new(value),
            var_type,
        }
    }
}
impl LayoutDeclaration {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self {
            declarations,
        }
    }
}

impl LambdaFunctionBinding {
    pub fn new(parameters: Vec<Token>, expressions: Vec<Expression>) -> Self {
        Self {
            parameters,
            expressions,
        }
    }
}

impl DirectBindings {
    pub fn new(names: Vec<Token>) -> Self {
        Self {
            names,
        }
    }
}

impl BindingDeclaration {
    pub fn new(name: Token, binding: LayoutItemProp) -> Self {
        Self {
            name,
            binding,
        }
    }
}

impl LayoutItemDeclaration {
    pub fn new(name: Token, bindings: Vec<BindingDeclaration>, string_literal: Option<Token>, declarations: Vec<Declaration>,) -> Self {
        Self {
            name,
            bindings,
            string_literal,
            declarations,
        }
    }

    pub fn with_string_literal(&mut self, string_literal: Token) -> &Self {
        self.string_literal = Some(string_literal);
        self
    }
}
impl SignalDeclaration {
    pub fn new(name: Token, parameters: Vec<Token>) -> Self {
        Self {
            name,
            parameters,
        }
    }
}
impl MethodDeclaration {
    pub fn new(name: Token, parameters: Vec<Token>, block: Statement) -> Self {
        Self {
            name,
            parameters,
            block: Box::new(block),
            decorator: None,
        }
    }

    pub fn with_decorator(&mut self, decorator: Decorator) -> &Self {
        self.decorator = Some(decorator);
        self
    }
}

impl ExportDeclaration {
    pub fn new(exported: Declaration) -> Self {
        Self {
            exported: Box::new(exported),
        }
    }
}

impl EnumDeclaration {
    pub fn new(name: Token, members: Vec<Token>) -> Self {
        Self {
            name,
            members,
        }
    }
}