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
    KwFn, // "fn"
    KwDo, // "do"
    KwWhile, // "while"
    KwBreak, // "break"
    KwContinue, // "continue"

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

#[derive(Debug)]
pub struct Decorator {
    name: Box<Token>,
    parameters: Vec<Token>,
}

#[derive(Debug)]
pub enum VarType {
    Let,
    Const,
}

#[derive(Debug)]
pub enum Declaration {
    Component { name: Option<Token>, parameters: Vec<Token>, declarations: Vec<Declaration> },
    Var { name: Token, value: Expression, var_type: VarType },
    Layout { /* TODO: Figure out what layout members are exactly */},
    Signal { name: Token, parameters: Vec<Token> },
    Method { decorator: Option<Decorator>, method: Statement, parameters: Vec<Token>, block: Statement },
}

#[derive(Debug)]
pub enum Statement {
    Block { statements: Vec<Statement> },
    Assign { name: Box<Token>, expression: Box<Expression> },
    Var { name: Box<Token>, expression: Box<Expression>, var_type: VarType },
    Break {},
    Continue {},
    /* FunctionDeclaration { name: Box<Token>, parameters: Vec<Token> }, */
    ForGeneric { var_name: Box<Token>, iter_name: Box<Token>, iterable: Box<Expression>, block: Box<Statement> },
    If { condition: Box<Expression>, if_block: Box<Statement>, else_stat: Option<Box<Statement>> },
    Expression { expression: Box<Expression> }
}

#[derive(Debug)]
pub enum Expression {
    Literal { token: Box<Token> },
    Binary { lhs: Box<Expression>, op: Box<Token>, rhs: Box<Expression> },
    Unary { op: Box<Token>, rhs: Box<Expression> },
    FunctionCall { name: Box<Token>, arguments: Vec<Expression> },
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