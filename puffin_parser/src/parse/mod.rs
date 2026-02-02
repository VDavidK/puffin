use puffin_ast::{Ast, Token, TokenType, Statement, Expression};
use puffin_ast::span::Span;
use puffin_ast::Statement::BreakStatement;
use crate::lex::{PuffinLexer, LexerError };

fn get_op_precedence(ty: TokenType) -> usize {
    match ty {
        TokenType::Plus | TokenType::Minus => 1,
        TokenType::Star | TokenType::Slash | TokenType::Percent => 2,
        TokenType::GreaterThan | TokenType::GreaterOrEqual | TokenType::LessThan | TokenType::LessOrEqual => 3,
        TokenType::IsEqualTo | TokenType::IsNotEqualTo => 4,
        TokenType::KwAnd => 5,
        TokenType::KwOr => 6,
        _ => 0,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error(transparent)]
    FileNotFoundError(#[from] std::io::Error),
    #[error(transparent)]
    LexerError(#[from] LexerError),
    #[error("Expected binary operator at {0}")]
    ExpectedBinaryOperatorError(Span),
    #[error("Expected literal at {0}")]
    ExpectedLiteralError(Span),
    #[error("Expected unary operator at {0}")]
    ExpectedUnaryOperatorError(Span),
    #[error("Expected one of {expected:?} at {span} found '{received}'")]
    UnexpectedTokenError {
        span: Span,
        expected: Vec<TokenType>,
        received: TokenType,
    },
    #[error("Unexpected end of file")]
    UnexpectedEofError(),
}

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct PuffinParser<'a> {
    lexer: PuffinLexer<'a>,
    current_token: Option<Token>,
}

impl<'a> PuffinParser<'a> {
    pub(crate) fn new(src: &'a str, src_name: &'a str) -> Self {
        PuffinParser {
            lexer: PuffinLexer::new(src, src_name),
            current_token: None,
        }
    }

    fn pos(&self) -> Span {
        self.current_token.as_ref().map(|t| t.span.clone()).unwrap_or_default()
    }

    fn peek(&mut self) -> Result<Option<&Token>, ParserError> {
        if self.current_token.is_some() {
            Ok(self.current_token.as_ref())
        } else {
            if let Some(tok) = self.lexer.next() {
                self.current_token = Some(tok?);
                Ok(self.current_token.as_ref())
            } else {
                Ok(None)
            }
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, ParserError> {
        if self.current_token.is_some() {
            let tok = self.current_token.take().unwrap();
            self.current_token = None;
            Ok(Some(tok))
        } else {
            if let Some(tok) = self.lexer.next() {
                Ok(Some(tok?))
            } else {
                Ok(None)
            }
        }
    }

    pub fn expect(&mut self, types: &[TokenType]) -> Result<Option<&Token>, ParserError> {
        let res = self.next_token()?.ok_or(ParserError::UnexpectedEofError())?;
        if types.contains(&res.ty) {
            Ok(self.current_token.as_ref())
        } else {
            Err(ParserError::UnexpectedTokenError{ span: res.span.to_owned(), expected: types.iter().cloned().collect::<Vec<_>>(), received: res.ty })
        }
    }

    pub(crate) fn run(mut self) -> Result<Ast, ParserError> {
        let mut ast = Ast::new();
        while let Some(token) = self.peek()? {
            let stat = match token.ty {
                TokenType::KwIf => self.if_stat(),
                _ => Ok(BreakStatement {})
            };
            ast.statements.push(Box::new(stat?));
        }
        dbg!(&ast);
        Ok(ast)
    }

    fn if_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwIf])?;
        let condition = self.expression()?;
        self.expect(&[TokenType::LeftBrace])?;
        let if_block = self.block()?;
        self.expect(&[TokenType::RightBrace])?;
        let stat = Statement::IfStatement {
            condition: Box::new(condition),
            if_block: Box::new(if_block),
            else_stat: None,
        };
        Ok(stat)
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        Ok(self.binary_expression(0)?)
    }

    fn binary_expression(&mut self, precedence: usize) -> Result<Expression, ParserError> {
        let mut expr: Expression = self.unary()?;
        loop {
            let pos = self.pos();
            let op = self.peek()?.ok_or(ParserError::ExpectedBinaryOperatorError(pos))?.clone();
            let prec = get_op_precedence(op.ty);
            if prec == 0 || prec <= precedence {
                break Ok(expr)
            }
            // Consume the peeked token
            self.next_token()?;

            let rhs = self.binary_expression(prec)?;
            expr = Expression::Binary {  lhs: Box::new(expr), op: Box::new(op), rhs: Box::new(rhs) };
        }
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        if let Some(tok) = self.peek()? {
            if matches!(tok.ty, TokenType::Plus | TokenType::Minus | TokenType::KwNot) {
                let expr = Expression::Unary {
                    op: Box::new(tok.clone()),
                    rhs: Box::new(self.unary()?),
                };
                Ok(expr)
            } else {
                self.literal()
            }
        } else {
            Err(ParserError::ExpectedUnaryOperatorError(pos))
        }
    }

    fn literal(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        let tok = self.next_token()?.as_ref().ok_or(ParserError::ExpectedLiteralError(pos))?.clone();
        let expr = Expression::Literal {
            token: Box::new(tok)
        };

        Ok(expr)
    }

    fn block(&mut self) -> Result<Statement, ParserError> {
        let stat = Statement::BlockStatement { statements: vec![] };
        Ok(stat)
    }
}
