use crate::token::{Token};
use crate::expression::{Expression};
use crate::statement::{Statement};
use crate::{VarType};
use crate::markup::Markup;

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
    pub markup: Vec<Markup>,
    pub name: Option<Token>,
    pub parameters: Vec<Token>,
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
    pub fn new(markup: Vec<Markup>, parameters: Vec<Token>) -> Self {
        Self {
            name: None,
            markup,
            parameters,
        }
    }

    pub fn with_name(mut self, name: Option<Token>) -> Self {
        self.name = name;
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

impl From<ComponentDeclaration> for Declaration {
    fn from(value: ComponentDeclaration) -> Self {
        Declaration::Component(value)
    }
}
impl From<VarDeclaration> for Declaration {
    fn from(value: VarDeclaration) -> Self {
        Declaration::Var(value)
    }
}
impl From<LayoutDeclaration> for Declaration {
    fn from(value: LayoutDeclaration) -> Self {
        Declaration::Layout(value)
    }
}
impl From<SignalDeclaration> for Declaration {
    fn from(value: SignalDeclaration) -> Self {
        Declaration::Signal(value)
    }
}
impl From<MethodDeclaration> for Declaration {
    fn from(value: MethodDeclaration) -> Self {
        Declaration::Method(value)
    }
}
impl From<RequireDeclaration> for Declaration {
    fn from(value: RequireDeclaration) -> Self {
        Declaration::Require(value)
    }
}
impl From<UseDeclaration> for Declaration {
    fn from(value: UseDeclaration) -> Self {
        Declaration::Use(value)
    }
}
impl From<ExportDeclaration> for Declaration {
    fn from(value: ExportDeclaration) -> Self {
        Declaration::Export(value)
    }
}
impl From<EnumDeclaration> for Declaration {
    fn from(value: EnumDeclaration) -> Self {
        Declaration::Enum(value)
    }
}