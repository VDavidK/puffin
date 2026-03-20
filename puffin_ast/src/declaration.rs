use crate::token::{Token};
use crate::expression::{Expression};
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
}

#[derive(Debug)]
pub struct LayoutDeclaration {
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
    Export(ExportDeclaration),
    Enum(EnumDeclaration),
    Error(ErrorDeclaration),
}

impl TryInto<ComponentDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<ComponentDeclaration, ()> {
        match self {
            Declaration::Component(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a ComponentDeclaration> for &'a Declaration {

    type Error = ();
    fn try_into(self) -> Result<&'a ComponentDeclaration, ()> {
        match self {
            Declaration::Component(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl TryInto<VarDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<VarDeclaration, ()> {
        match self {
            Declaration::Var(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl TryInto<LayoutDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<LayoutDeclaration, ()> {
        match self {
            Declaration::Layout(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<SignalDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<SignalDeclaration, ()> {
        match self {
            Declaration::Signal(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<MethodDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<MethodDeclaration, ()> {
        match self {
            Declaration::Method(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl<'a> TryInto<&'a MethodDeclaration> for &'a Declaration {

    type Error = ();
    fn try_into(self) -> Result<&'a MethodDeclaration, ()> {
        match self {
            Declaration::Method(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<RequireDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<RequireDeclaration, ()> {
        match self {
            Declaration::Require(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<UseDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<UseDeclaration, ()> {
        match self {
            Declaration::Use(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<ExportDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<ExportDeclaration, ()> {
        match self {
            Declaration::Export(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<EnumDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<EnumDeclaration, ()> {
        match self {
            Declaration::Enum(c) => Ok(c),
            _ => Err(()),
        }
    }
}
impl TryInto<ErrorDeclaration> for Declaration {
    type Error = ();
    fn try_into(self) -> Result<ErrorDeclaration, ()> {
        match self {
            Declaration::Error(c) => Ok(c),
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

impl From<ErrorDeclaration> for Declaration {
    fn from(value: ErrorDeclaration) -> Self {
        Declaration::Error(value)
    }
}
impl From<ComponentDeclaration> for Declaration {
    fn from(value: ComponentDeclaration) -> Self {
        Declaration::Component(value)
    }
}
impl From<ConstructorDeclaration> for Declaration {
    fn from(value: ConstructorDeclaration) -> Self {
        Declaration::Constructor(value)
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