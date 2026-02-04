use crate::lex::LexerError::UnterminatedStringLiteral;
use puffin_ast::{span::Span, Token, TokenType, position::Position};

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
            '+' if self.match_while("++") => self.token(TokenType::Increment),
            '+' if self.match_while("+=") => self.token(TokenType::IncrementAssign),
            '+' => self.simple_token(TokenType::Plus),
            '-' if self.match_while("--") => self.token(TokenType::Decrement),
            '-' if self.match_while("-=") => self.token(TokenType::DecrementAssign),
            '-' => self.simple_token(TokenType::Minus),
            '(' => self.simple_token(TokenType::LeftParen),
            ')' => self.simple_token(TokenType::RightParen),
            '{' => self.simple_token(TokenType::LeftBrace),
            '}' => self.simple_token(TokenType::RightBrace),
            '[' => self.simple_token(TokenType::LeftBracket),
            ']' => self.simple_token(TokenType::RightBracket),
            '*' if self.match_while("*=") => self.token(TokenType::MulAssign),
            '*' => self.simple_token(TokenType::Star),
            '/' if self.match_while("//") => {
                while let Some(c) = self.peek(0) && c != '\n' {
                    self.skip();
                }
                self.next()?
            }
            '/' if self.match_while("/=") => self.token(TokenType::DivAssign),
            '/' => self.simple_token(TokenType::Slash),
            '.' => self.simple_token(TokenType::Dot),
            ',' => self.simple_token(TokenType::Comma),
            ':' => self.simple_token(TokenType::Colon),
            ';' => self.simple_token(TokenType::Semicolon),
            '%' => self.simple_token(TokenType::Percent),
            '#' => self.simple_token(TokenType::Hash),
            '@' => self.simple_token(TokenType::At),
            '"' => {
                self.next_char();
                if let Err(err) = self.match_while_not('"', UnterminatedStringLiteral(self.start.clone())) {
                    Err(err)
                } else {
                    self.simple_token(TokenType::String)
                }
            }
            '=' if self.match_while("==") => self.token(TokenType::IsEqualTo),
            '=' if self.match_while("=>") => self.token(TokenType::Arrow),
            '=' => self.simple_token(TokenType::Assign),
            '!' if self.match_while("!=") => self.token(TokenType::IsNotEqualTo),
            '>' if self.match_while(">=") => self.token(TokenType::GreaterOrEqual),
            '>' => self.simple_token(TokenType::GreaterThan),
            '<' if self.match_while("<=") => self.simple_token(TokenType::LessOrEqual),
            '<' => self.simple_token(TokenType::LessThan),
            c if c.is_alphabetic() => {
                while let Some(c) = self.peek(0) && (c.is_ascii_alphanumeric() || c == '_') {
                    self.next_char();
                }
                match self.lexeme() {
                    "or" => self.token(TokenType::KwOr),
                    "not" => self.token(TokenType::KwNot),
                    "and" => self.token(TokenType::KwAnd),
                    "true" => self.token(TokenType::KwTrue),
                    "false" => self.token(TokenType::KwFalse),
                    "if" => self.token(TokenType::KwIf),
                    "else" => self.token(TokenType::KwElse),
                    "match" => self.token(TokenType::KwMatch),
                    "for" => self.token(TokenType::KwFor),
                    "in" => self.token(TokenType::KwIn),
                    "layout" => self.token(TokenType::KwLayout),
                    "component" => self.token(TokenType::KwComponent),
                    "signal" => self.token(TokenType::KwSignal),
                    "let" => self.token(TokenType::KwLet),
                    "const" => self.token(TokenType::KwConst),
                    "export" => self.token(TokenType::KwExport),
                    "fn" => self.token(TokenType::KwFn),
                    "do" => self.token(TokenType::KwDo),
                    "while" => self.token(TokenType::KwWhile),
                    "break" => self.token(TokenType::KwBreak),
                    "continue" => self.token(TokenType::KwContinue),
                    _ => self.token(TokenType::Identifier),
                }
            },
            c if c.is_numeric() => {
                let mut dot_found = false;
                while let Some(c) = self.peek(0) && (c.is_numeric() || c == '.') {
                    if c == '.' {
                        if dot_found == true {
                            break;
                        }
                        dot_found = true;
                    }
                    self.next_char();
                }
                self.token(if dot_found { TokenType::Float } else { TokenType::Integer })
            },
            ' ' | '\t' | '\r' | '\n' => {
                self.next_char();
                return self.next();
            }
            c => Err(LexerError::UnrecognizedCharacter(self.start.clone().with_snippet(self.src, self.src_name, 2), c))
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

    /// Moves the start and end pointers forward by one.
    /// This is not intended for skipping in the middle of a lexeme,
    /// but rather for skipping comments.
    fn skip(&mut self) {
        self.start.move_forward(self.src);
        self.end.move_forward(self.src);
    }

    fn advance(&mut self) {
        self.start = self.end.clone();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.src
            .chars()
            .nth(self.end.idx() + offset)
    }

    fn match_while(&mut self, pattern: &str) -> bool {
        for i in 0..pattern.len() {
            if self.peek(i) != pattern.chars().nth(i) {
                return false;
            }
        }
        self.end.move_forward_by(self.src, pattern.len());
        true
    }

    fn match_while_not<T>(&mut self, c: char, err: T) -> Result<(), T> {
        while let Some(ch) = self.peek(0) {
            if ch == c {
                return Ok(());
            } else {
                self.next_char();
            }
        }
        Err(err)
    }

    fn simple_token(&mut self, ty: TokenType) -> Result<Token, LexerError> {
        self.next_char();
        self.token(ty)
    }

    fn token(&mut self, ty: TokenType) -> Result<Token, LexerError> {
        let tok = Token::new(
            self.lexeme().to_string(),
            Span::from_positions(self.start.clone(), self.end.clone()),
                /*.with_snippet(&self.src.to_owned(), &self.src_name.to_owned(), 1),*/
            ty
        );
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