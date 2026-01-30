mod position;
mod span;
mod snippet;

use position::Position;
use crate::lex::span::Span;

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Unrecognized character '{1}' found at '{0}'")]
    UnrecognizedCharacter(Position, char),
    #[error("Unterminated string literal found at '{0}'")]
    UnterminatedStringLiteral(Position),
    #[error("Unterminated block comment found at '{0}'")]
    UnterminatedBlockComment(Position),
}

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
pub struct Token {
    span: Span,
    lexeme: String,
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

#[derive(Debug)]
pub(crate) struct PuffinLexer<'a> {
    start: Position,
    end: Position,
    src: &'a str,
    src_name: &'a str,
}

impl<'a> Iterator for PuffinLexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();

        Some(match self.peek(0)? {
            '+' => self.simple_token(TokenType::Plus),
            c => Err(LexerError::UnrecognizedCharacter(self.start.clone().with_snippet(self.src, self.src_name, 2), c)),
        })
    }
}

impl<'a> PuffinLexer<'a> {
    pub fn new(src: &'a str, src_name: &'a str) -> Self {
        PuffinLexer {
            start: Position::new(),
            end: Position::new(),
            src,
            src_name,
        }
    }

    fn advance(&mut self) {
        self.start = self.end.clone();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.src
            .chars()
            .nth(self.end.idx() + offset)
    }

    fn simple_token(&mut self, ty: TokenType) -> Result<Token, LexerError> {
        self.next_char();
        self.token(ty)
    }

    fn token(&mut self, ty: TokenType) -> Result<Token, LexerError> {
        let tok = Token {
            ty,
            lexeme: self.lexeme().to_string(),
            span: Span::from_positions(self.start.clone(), self.end.clone()),
        };
        self.advance();
        Ok(tok)
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek(0);
        self.end.move_forward(self.src);
        c
    }

    fn lexeme(&self) -> &str {
        &self.src[self.start.idx()..self.end.idx()]
    }
}