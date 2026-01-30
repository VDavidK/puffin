pub mod position;
pub mod span;
mod snippet;

use crate::span::Span;

#[derive(Debug, Copy, Clone)]
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
    pub(crate) span: Span,
    ty: TokenType,
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Token {:?} '{}' at {}", self.ty, self.lexeme, self.span))
    }
}

pub(crate) enum Statement {
    BlockStatement { statements: Vec<Box<Statement>> },
    ComponentDeclarationStatement { name: Box<Token>, block: Box<Statement> },
}

pub(crate) enum Expression {}