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
pub enum VarType {
    Let,
    Const,
}

#[derive(Debug)]
pub enum Statement {
    BlockStatement { statements: Vec<Box<Statement>> },
    ComponentDeclarationStatement { name: Box<Token>, parameters: Vec<Box<Token>>, block: Box<Statement> },
    ComponentBodyDeclaration { statements: Vec<Box<Statement>>, layout: Box<Statement> },
    AssignStatement { name: Box<Token>, expression: Box<Expression> },
    VarDeclarationStatement { name: Box<Token>, expression: Box<Expression>, var_type: VarType },
    BreakStatement {},
    ContinueStatement {},
    LayoutStatement { name: Box<Token>, block: Box<Statement> },
    MethodDeclaration { name: Box<Token>, parameters: Vec<Box<Token>>, block: Box<Statement> },
    Decorator { name: Box<Token>, args: Vec<Box<Token>> },
    DecoratedMethodDeclaration { decorator: Box<Statement>, method: Box<Statement> },
    ForGeneric { var_name: Box<Token>, iter_name: Box<Token>, iterable: Box<Expression>, block: Box<Statement> },
    IfStatement { condition: Box<Expression>, if_block: Box<Statement>, else_stat: Option<Box<Statement>> },
}

#[derive(Debug)]
pub enum Expression {
    Literal { token: Box<Token> },
    Binary { lhs: Box<Expression>, op: Box<Token>, rhs: Box<Expression> },
    Unary { op: Box<Token>, rhs: Box<Expression> },
    FunctionCall { name: Box<Token>, arguments: Vec<Box<Expression>> },
}

#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<Box<Statement>>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}