#[derive(Debug)]
pub(crate) enum TokenType {
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


#[derive(Debug)]
pub(crate) struct PuffinLexer {}

impl PuffinLexer {
    pub fn new() -> Self {
        PuffinLexer {}
    }
}