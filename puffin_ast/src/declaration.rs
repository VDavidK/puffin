use crate::token::{Token};
use crate::expression::{BinaryExpression, Expression};
use crate::statement::{Statement};
use crate::{VarType};
use crate::markup::Markup;

#[derive(Debug)]
pub struct ComponentDeclaration {
    pub name: Token,
    pub constructor: Option<Box<Declaration>>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct ConstructorDeclaration {
    pub parameters: Vec<Token>,
    pub block: Box<Statement>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub name: Token,
    pub value: Box<Expression>,
    pub var_type: VarType,
    pub exported: bool,
}

#[derive(Debug)]
pub struct LayoutDeclaration {
    pub name: Option<Token>,
    pub markup: Box<Markup>,
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
pub struct Decorator {
    pub name: Token,
    pub parameters: Vec<Token>,
}

#[derive(Debug)]
pub struct EnumDeclaration {
    pub name: Token,
    pub members: Vec<Token>,
    pub exported: bool,
}

#[derive(Debug)]
pub struct ErrorDeclaration {
    pub members: Vec<Token>,
}

#[derive(Debug)]
pub enum Declaration {
    Component(ComponentDeclaration),
    Constructor(ConstructorDeclaration),
    Var(VarDeclaration),
    Layout(LayoutDeclaration),
    Signal(SignalDeclaration),
    Method(MethodDeclaration),
    Require(RequireDeclaration),
    Use(UseDeclaration),
    Enum(EnumDeclaration),
    Error(ErrorDeclaration),
}
impl TryFrom<Declaration> for ComponentDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Component(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for VarDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Var(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for LayoutDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Layout(l) => Ok(l),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for SignalDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Signal(s) => Ok(s),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for MethodDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Method(m) => Ok(m),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Declaration> for &'a MethodDeclaration {
    type Error = ();
    fn try_from(value: &'a Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Method(m) => Ok(m),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for RequireDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Require(r) => Ok(r),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for UseDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Use(u) => Ok(u),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for EnumDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Enum(e) => Ok(e),
            _ => Err(()),
        }
    }
}

impl TryFrom<Declaration> for ErrorDeclaration {
    type Error = ();
    fn try_from(value: Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Error(e) => Ok(e),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Declaration> for &'a ErrorDeclaration {
    type Error = ();
    fn try_from(value: &'a Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Error(e) => Ok(e),
            _ => Err(()),
        }
    }
}

impl Decorator {
    pub fn new(name: Token, parameters: Vec<Token>) -> Self {
        Self {
            name,
            parameters,
        }
    }
}

impl ErrorDeclaration {
    pub fn new(members: Vec<Token>) -> Self {
        Self {
            members,
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
    pub fn new(name: Token, declarations: Vec<Declaration>) -> Self {
        Self {
            name,
            constructor: None,
            declarations,
        }
    }

    pub fn with_constructor(mut self, constructor: Declaration) -> Self {
        self.constructor = Some(Box::new(constructor));
        self
    }
}

impl ConstructorDeclaration {
    pub fn new(parameters: Vec<Token>, block: Statement) -> Self {
        Self {
            parameters,
            block: Box::new(block),
        }
    }
}
impl VarDeclaration {
    pub fn new_const(name: Token, value: Expression, exported: bool) -> Self {
        Self {
            name,
            value: Box::new(value),
            var_type: VarType::Const,
            exported,
        }
    }

    pub fn new_let(name: Token, value: Expression, exported: bool) -> Self {
        Self {
            name,
            value: Box::new(value),
            var_type: VarType::Let,
            exported,
        }
    }
}
impl LayoutDeclaration {
    pub fn new(markup: Markup, parameters: Vec<Token>) -> Self {
        Self {
            name: None,
            markup: Box::new(markup),
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

    pub fn with_decorator(mut self, decorator: Decorator) -> Self {
        self.decorator = Some(decorator);
        self
    }
}

impl EnumDeclaration {
    pub fn new(name: Token, members: Vec<Token>, exported: bool) -> Self {
        Self {
            name,
            members,
            exported,
        }
    }
}

impl<'a> TryFrom<&'a Declaration> for &'a VarDeclaration {
    type Error = ();
    fn try_from(value: &'a Declaration) -> Result<Self, Self::Error> {
        match value {
            Declaration::Var(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<ErrorDeclaration> for Declaration {
    fn from(d: ErrorDeclaration) -> Self {
        Declaration::Error(d)
    }
}

impl From<ConstructorDeclaration> for Declaration {
    fn from(d: ConstructorDeclaration) -> Self {
        Declaration::Constructor(d)
    }
}

impl From<ComponentDeclaration> for Declaration {
    fn from(d: ComponentDeclaration) -> Self {
        Declaration::Component(d)
    }
}

impl From<VarDeclaration> for Declaration {
    fn from(d: VarDeclaration) -> Self {
        Declaration::Var(d)
    }
}

impl From<LayoutDeclaration> for Declaration {
    fn from(d: LayoutDeclaration) -> Self {
        Declaration::Layout(d)
    }
}

impl From<SignalDeclaration> for Declaration {
    fn from(d: SignalDeclaration) -> Self {
        Declaration::Signal(d)
    }
}

impl From<MethodDeclaration> for Declaration {
    fn from(d: MethodDeclaration) -> Self {
        Declaration::Method(d)
    }
}

impl From<RequireDeclaration> for Declaration {
    fn from(d: RequireDeclaration) -> Self {
        Declaration::Require(d)
    }
}

impl From<UseDeclaration> for Declaration {
    fn from(d: UseDeclaration) -> Self {
        Declaration::Use(d)
    }
}

impl From<EnumDeclaration> for Declaration {
    fn from(d: EnumDeclaration) -> Self {
        Declaration::Enum(d)
    }
}
