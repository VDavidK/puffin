use puffin_ast::{Ast, Token, TokenType, Statement, Expression, Declaration, VarType, Decorator, Component, Method, Var, Signal};
use puffin_ast::span::Span;
use crate::lex::{PuffinLexer, LexerError };
use crate::parse::ParserError::ExpectedDeclarationError;

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
    #[error("Expected literal or parenthesis expression at {0}")]
    ExpectedLiteralOrParenError(Span),
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
    #[error("Expected declaration at {0}")]
    ExpectedDeclarationError(Span),
    #[error("Expected identifier at {0}")]
    ExpectedIdentifierError(Span),
    #[error("Expected statement at {0}")]
    ExpectedStatementError(Span),
    #[error("Expected method declaration at {0}")]
    ExpectedMethodError(Span),
    #[error("Expected let/const at {0}, received {1}")]
    ExpectedVarTypeError(Span, TokenType),
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
        self.peek()?;
        if self.current_token.is_some() {
            let tok = self.current_token.take().unwrap();
            self.current_token = None;
            Ok(Some(tok))
        } else {
            Ok(None)
        }
    }

    pub fn expect(&mut self, types: &[TokenType]) -> Result<Token, ParserError> {
        let res = self.next_token()?.ok_or(ParserError::UnexpectedEofError())?;
        if types.contains(&res.ty) {
            Ok(res)
        } else {
            Err(ParserError::UnexpectedTokenError{
                span: res.span.to_owned(),
                expected: types.iter().cloned().collect::<Vec<_>>(),
                received: res.ty
            })
        }
    }

    pub(crate) fn run(mut self) -> Result<Ast, ParserError> {
        let mut decls = Vec::new();
        while self.peek()?.is_some() {
            decls.push(self.decl()?);
        }
        let ast = Ast::new(decls);
        dbg!(&ast);
        Ok(ast)
    }

    fn decl(&mut self) -> Result<Declaration, ParserError> {
        let pos = self.pos();
        let decl  = match self.peek()?.ok_or(ExpectedDeclarationError(pos))?.ty {
            TokenType::KwLet | TokenType::KwConst => self.var_decl()?,
            TokenType::KwSignal => self.signal()?,
            TokenType::At => self.decorated_method()?,
            TokenType::KwFn => self.method()?,
            TokenType::KwComponent => self.component()?,
            _ => return Err(ParserError::ExpectedDeclarationError(self.pos()))
        };
        Ok(decl)
    }

    fn decls(&mut self) -> Result<Vec<Declaration>, ParserError> {
        let mut decls = Vec::new();
        while let Ok(decl) = self.decl() {
            decls.push(decl);
        }
        Ok(decls)
    }

    fn component(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwComponent])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?.ok_or(Vec::<Token>::new()).unwrap();
        self.expect(&[TokenType::LeftBrace])?;
        let decls = self.decls()?;
        self.expect(&[TokenType::RightBrace])?;

        let decl = Declaration::Component(Component {
            name: Some(name),
            parameters: params,
            declarations: decls,
        });
        Ok(decl)
    }

    fn decorated_method(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::At])?;
        let decorator_name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?.ok_or(Vec::<Token>::new()).unwrap();
        let mut method = self.method()?;
        if let Declaration::Method(m) = &mut method {
            m.decorator = Some(Decorator {
                name: decorator_name,
                parameters: params,
            });
            Ok(method)
        } else {
            Err(ParserError::ExpectedMethodError(self.pos().clone()))
        }
    }

    fn method(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwFn])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?.ok_or(Vec::<Token>::new()).unwrap();
        self.expect(&[TokenType::LeftBrace])?;
        let decl = Declaration::Method(Method {
            name,
            parameters: params,
            decorator: None,
            block: self.block()?,
        });
        self.expect(&[TokenType::RightBrace])?;
        Ok(decl)
    }

    fn parameters(&mut self) -> Result<Option<Vec<Token>>, ParserError> {
        if self.peek()?.is_some_and(|f| f.ty == TokenType::LeftParen) {
            self.next_token()?;
            let params = self.name_list()?;
            self.expect(&[TokenType::RightParen])?;
            Ok(Some(params))
        } else {
            Ok(Some(Vec::new()))
        }
    }

    fn var_decl(&mut self) -> Result<Declaration, ParserError> {
        let ty = match self.expect(&[TokenType::KwConst, TokenType::KwLet])?.ty {
            TokenType::KwConst => VarType::Const,
            TokenType::KwLet => VarType::Let,
            t => return Err(ParserError::ExpectedVarTypeError(self.pos(), t))
        };
        let name = self
            .expect(&[TokenType::Identifier])?
            .clone();
        self.expect(&[TokenType::Assign])?;
        let decl = Declaration::Var(Var {
            name,
            value: self.expression()?,
            var_type: ty,
        });
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    fn signal(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwSignal])?;
        let pos = self.pos();
        let name = self.next_token()?.ok_or(ParserError::ExpectedIdentifierError(pos))?;
        let params = self.parameters()?.ok_or(Vec::<Token>::new()).unwrap();
        let decl = Declaration::Signal(Signal {
            name,
            parameters: params,
        });
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    fn name_list(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = Vec::new();
        while let Some(tok) = self.peek()? && tok.ty == TokenType::Identifier {
            names.push(tok.clone());
            self.next_token()?;
            if self.peek()?.is_some_and(|t| t.ty != TokenType::Comma) {
                break;
            } else {
                self.next_token()?;
            }
        }
        Ok(names)
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        let pos = self.pos();
        let tok = self
            .next_token()?
            .ok_or(ParserError::ExpectedDeclarationError(pos.clone()))?;
        let stat = match tok.ty {
            TokenType::KwIf => self.if_stat(),
            _ => return Err(ParserError::ExpectedStatementError(pos)),
        }?;
        Ok(stat)
    }

    fn if_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwIf])?;
        let condition = self.expression()?;
        self.expect(&[TokenType::LeftBrace])?;
        let if_block = self.block()?;
        self.expect(&[TokenType::RightBrace])?;
        let stat = Statement::If {
            condition: Box::new(condition),
            if_block: Box::new(if_block),
            else_stat: None,
        };
        Ok(stat)
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        Ok(self.binary(0)?)
    }

    fn binary(&mut self, precedence: usize) -> Result<Expression, ParserError> {
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

            let rhs = self.binary(prec)?;
            expr = Expression::Binary {  lhs: Box::new(expr), op: Box::new(op), rhs: Box::new(rhs) };
        }
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        if let Some(tok) = self.peek()? {
            match tok.ty {
                TokenType::Plus | TokenType::Minus | TokenType::KwNot => {
                    let expr = Expression::Unary {
                        op: Box::new(tok.clone()),
                        rhs: Box::new(self.unary()?),
                    };
                    Ok(expr)
                },
                _ => self.primary(),
            }
        } else {
            Err(ParserError::ExpectedUnaryOperatorError(pos))
        }
    }

    fn call_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        let tok = self.next_token()?.as_ref().ok_or(ParserError::ExpectedLiteralOrParenError(pos))?.clone();
        if let Some(par) = self.peek()? && par.ty == TokenType::LeftParen {
            self.next_token()?;
            let expr = self.expression()?;
            self.expect(&[TokenType::RightParen])?;
            Ok(expr)
        } else {
            let expr = Expression::Literal {
                token: Box::new(tok)
            };
            Ok(expr)
        }
    }

    fn block(&mut self) -> Result<Statement, ParserError> {
        let stat = Statement::Block { statements: vec![] };
        Ok(stat)
    }
}
