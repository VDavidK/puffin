pub mod position;
pub mod span;
mod snippet;

use crate::span::Span;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    KwAnd, // "and"
    KwOr, // "or"
    KwNot, // "not"
    KwTrue, // "true"
    KwFalse, // "false"
    KwIf, // "if"
    KwElse, // "else"
    KwMatch, // "match"
    KwFor, // "for"
    KwIn, // "in"
    KwLayout, // "layout"
    KwComponent, // "component"
    KwSignal, // "signal"
    KwLet, // "let"
    KwConst, // "const"
    KwExport, // "export"
    KwRequire, // "require"
    KwUse, // "use"
    KwFn, // "fn"
    KwDo, // "do"
    KwWhile, // "while"
    KwBreak, // "break"
    KwContinue, // "continue"
    KwThrow, // "throw"
    KwReturn, // "return"
    KwStyle, // "style"
    KwWith, // "with"

    LeftBrace, // "{"
    RightBrace, // "}"
    LeftParen, // ")"
    RightParen, // "("
    LeftBracket, // "["
    RightBracket, // "]"
    Plus, // "+"
    Minus, // "-"
    Star, // "*"
    Slash, // "/"
    Dot, // "."
    Comma, // ","
    Colon, // ":"
    Semicolon, // ";"
    Percent, // "%"
    Hash, // "#"
    At, // "@"
    Arrow, // "=>"
    Increment, // "++"
    Decrement, // "--"
    IncrementAssign, // "+="
    DecrementAssign, // "-="
    MulAssign, // "*="
    DivAssign, // "/="
    Assign, // "="
    IsEqualTo, // "=="
    IsNotEqualTo, // "!="
    GreaterThan, // ">"
    LessThan, // "<"
    GreaterOrEqual, // ">="
    LessOrEqual, // "<="

    Integer,
    Float,
    String,
    Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    lexeme: String,
    pub span: Span,
    pub ty: TokenType,
}

impl Token {
    pub fn new(lex: impl AsRef<str>, span: Span, ty: TokenType) -> Self {
        Self {
            span,
            lexeme: lex.as_ref().to_owned(),
            ty
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Token {:?} '{}' at {}", self.ty, self.lexeme, self.span))
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

#[derive(Debug)]
pub enum VarType {
    Let,
    Const,
}

#[derive(Debug)]
pub struct Component {
    pub name: Option<Token>,
    pub parameters: Vec<Token>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub value: Expression,
    pub var_type: VarType,
}

#[derive(Debug)]
pub struct Layout {
    /* TODO: Figure out what layout members are exactly */
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct LayoutItemProp {
    pub event_name: Token,
    pub actions: Vec<Expression>,
}

impl LayoutItemProp {
    pub fn new(event_name: Token, actions: Vec<Expression>) -> Self {
        Self {
            event_name,
            actions
        }
    }
}

#[derive(Debug)]
pub struct LayoutItem {
    pub name: Token,
    pub props: Vec<LayoutItemProp>,
    pub string_literal: Option<Token>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Signal {
    pub name: Token,
    pub parameters: Vec<Token>,
}

#[derive(Debug)]
pub struct Method {
    pub decorator: Option<Decorator>,
    pub name: Token,
    pub parameters: Vec<Token>,
    pub block: Statement,
}

#[derive(Debug)]
pub struct Decorator {
    pub name: Token,
    pub parameters: Vec<Token>,
}

#[derive(Debug)]
pub enum Declaration {
    Component(Component),
    Var(Var),
    Layout(Layout),
    LayoutItem(LayoutItem),
    Signal(Signal),
    Method(Method),
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct AssignStatement {
    pub name: Token,
    pub expression: Box<Expression>
}

#[derive(Debug)]
pub struct VarStatement {
    pub name: Token,
    pub expression: Box<Expression>,
    pub var_type: VarType
}

#[derive(Debug)]
pub struct BreakStatement {}

#[derive(Debug)]
pub struct ContinueStatement {}

/*
#[derive(Debug)]
pub struct FunctionDeclaration
 { name: Box<Token>, parameters: Vec<Token> }
*/

#[derive(Debug)]
pub struct ForStatement {
    pub var_name: Token,
    pub iterable: Box<Expression>,
    pub end_range: Option<Box<Expression>>,
    pub block: Box<Statement>
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub if_block: Box<Statement>,
    pub else_stat: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    Assign(AssignStatement),
    Var(VarStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    /* FunctionDeclaration { name: Box<Token>, parameters: Vec<Token> }, */
    For(ForStatement),
    If(IfStatement),
    Expression(ExpressionStatement),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub enum Expression {
    Literal { token: Token },
    Binary { lhs: Box<Expression>, op: Token, rhs: Box<Expression> },
    Unary { op: Token, rhs: Box<Expression> },
    FunctionCall { callee: Box<Expression>, arguments: Vec<Expression> },
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