use puffin_ast::{Ast, Token, TokenType, Statement, Expression, Declaration, VarType, Decorator, Component, Method, Var, Signal, ExpressionStatement, IfStatement, BlockStatement, AssignStatement};
use puffin_ast::Expression::{Binary, FunctionCall, Literal};
use puffin_ast::span::Span;
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

    /// Returns the position of the parser's current token if it is assigned, or a default position if it is ```None```.
    fn pos(&self) -> Span {
        self.current_token.as_ref().map(|t| t.span.clone()).unwrap_or_default()
    }

    /// Returns an indicator of whether the lexer's tokens have been exhausted.
    fn eof(&mut self) -> bool {
        self.peek().is_ok_and(|t| t.is_none())
    }

    /// Peeks the next token and returns it if it exists. Does not consume the currently active token.
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

    /// Peeks the next token and returns an indicator of whether its type matches ```expected```.
    fn peek_is(&mut self, expected: TokenType) -> Result<bool, ParserError> {
        Ok(self.peek()?.is_some_and(|f| f.ty == expected))
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

    /// Fetches the next token and errors if the token is ```None``` or if
    /// its type is not in ```types```, returning it otherwise.
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

    /// Runs the parser on the source file provided when it was initialized.
    pub(crate) fn run(mut self) -> Result<Ast, ParserError> {
        let mut decls = Vec::new();
        while !self.eof() {
            decls.push(self.decl()?);
        }
        let ast = Ast::new(decls);
        dbg!(&ast);
        Ok(ast)
    }

    /// decl ::= \<var_decl\>
    /// decl ::= \<signal_decl\>
    /// decl ::= \<decorated_method_decl\>
    /// decl ::= \<method_decl\>
    /// decl ::= \<component_decl\>
    fn decl(&mut self) -> Result<Declaration, ParserError> {
        let pos = self.pos();
        let decl  = match self.peek()?.ok_or(ParserError::ExpectedDeclarationError(pos))?.ty {
            TokenType::KwLet | TokenType::KwConst => self.var_decl()?,
            TokenType::KwSignal => self.signal()?,
            TokenType::At => self.decorated_method()?,
            TokenType::KwFn => self.method()?,
            TokenType::KwComponent => self.component()?,
            _ => return Err(ParserError::ExpectedDeclarationError(self.pos()))
        };
        Ok(decl)
    }

    /// \<component\> ::= "component", \<identifier\>, \<parameters\>, "{", {\<declaration\>}, "}"
    fn component(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwComponent])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?.ok_or(Vec::<Token>::new()).unwrap();
        self.expect(&[TokenType::LeftBrace])?;
        let mut decls = Vec::new();
        while !self.peek_is(TokenType::RightBrace)? {
            decls.push(self.decl()?);
        }
        self.expect(&[TokenType::RightBrace])?;

        let decl = Declaration::Component(Component {
            name: Some(name),
            parameters: params,
            declarations: decls,
        });
        Ok(decl)
    }

    /// \<decorated_method_decl\> ::= "@", \<identifier\>, \<parameters\>, \<method_decl\>
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

    /// \<parameters\> ::= "(", {\<identifier\>}, ")"
    fn parameters(&mut self) -> Result<Option<Vec<Token>>, ParserError> {
        if self.peek_is(TokenType::LeftParen)? {
            self.next_token()?;
            let params = self.name_list()?;
            self.expect(&[TokenType::RightParen])?;
            Ok(Some(params))
        } else {
            Ok(Some(Vec::new()))
        }
    }

    /// \<var_decl\> ::= "const" | "let", \<identifier\>, "=", \<expression\>, ";"
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

    /// \<signal\> ::= "signal", \<identifier\>, \<parameters\>, ";"
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

    /// <name_list> ::= \<identifier\>, {",", \<identifier\>}
    fn name_list(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = Vec::new();
        while self.peek_is(TokenType::Identifier)? {
            // Safe unwrap, as peek_is has verified the token's existence.
            names.push(self.next_token()?.unwrap().clone());
            // A comma indicates another identifier. Trailing commas are currently not allowed.
            if !self.peek_is(TokenType::Comma)? {
                break;
            } else {
                self.next_token()?;
            }
        }
        Ok(names)
    }

    /// \<statement\> ::= \<if_statement\>
    /// \<statement\> ::= \<expr_statement\>
    fn statement(&mut self) -> Result<Statement, ParserError> {
        let pos = self.pos();
        let stat = match self.peek()?.ok_or(ParserError::ExpectedDeclarationError(pos.clone()))?.ty {
            TokenType::KwIf => self.if_stat()?,
            TokenType::Identifier => self.expr_statement()?,
            _ => return Err(ParserError::ExpectedStatementError(pos))
        };
        Ok(stat)
    }

    /// \<if_statement\> ::= "if", \<expression\>, "{", \<block\>, "}", {"else", \<if_statement}>}, \[\"else\", \<block\>\]
    fn if_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwIf])?;
        let condition = self.expression()?;
        self.expect(&[TokenType::LeftBrace])?;
        let if_block = self.block()?;
        self.expect(&[TokenType::RightBrace])?;
        let else_block: Option<Box<Statement>> = match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
            TokenType::KwElse => {
                self.next_token()?;
                match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                    TokenType::KwIf => Some(Box::new(self.if_stat()?)),
                    _ => {
                        self.expect(&[TokenType::LeftBrace])?;
                        let stat = Some(Box::new(self.block()?));
                        self.expect(&[TokenType::RightBrace])?;
                        stat
                    }
                }
            },
            _ => None,
        };
        let stat = Statement::If(IfStatement {
            condition: Box::new(condition),
            if_block: Box::new(if_block),
            else_stat: else_block,
        });
        Ok(stat)
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        Ok(self.binary(0)?)
    }

    fn expr_statement(&mut self) -> Result<Statement, ParserError> {
        let pos = self.pos();
        let expr = match self.peek()?.ok_or(ParserError::ExpectedStatementError(pos.clone()))?.ty {
            TokenType::Identifier => {
                let name = self.expect(&[TokenType::Identifier])?;
                let tok = self.expect(&[TokenType::LeftParen, TokenType::Assign])?;
                match tok.ty {
                    TokenType::LeftParen => {
                        let expr = FunctionCall {
                            name,
                            arguments: {
                                let mut args = Vec::new();
                                while let Some(tok) = self.peek()? && tok.ty != TokenType::RightParen {
                                    args.push(self.expression()?);
                                }
                                args
                            },
                        };
                        self.expect(&[TokenType::RightParen])?;
                        expr
                    },
                    TokenType::Assign => Binary {
                        lhs: Box::new(Literal { token: name }),
                        op: tok.clone(),
                        rhs: Box::new(self.expression()?),
                    },
                    _ => return Err(ParserError::ExpectedStatementError(self.pos()))
                }
            },
            _ => return Err(ParserError::ExpectedStatementError(self.pos()))
        };
        self.expect(&[TokenType::Semicolon])?;
        let stat = Statement::Expression(ExpressionStatement{
            expression: Box::new(expr),
        });
        Ok(stat)
    }

/*    /// \<assignment\> ::= \<identifier\> "=" \<expression\>
    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::Assign])?;
        let expr = self.binary(0)?;
        self.expect(&[TokenType::Semicolon])?;
        let stat = Statement::Assign(AssignStatement {
            name,
            expression: Box::new(expr),
        });
        Ok(stat)
    }*/

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
            expr = Expression::Binary { lhs: Box::new(expr), op, rhs: Box::new(rhs) };
        }
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        if let Some(tok) = self.peek()? {
            match tok.ty {
                TokenType::Plus | TokenType::Minus | TokenType::KwNot => {
                    let expr = Expression::Unary {
                        op: tok.clone(),
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
        let name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::LeftParen])?;
        let mut args: Vec<Expression> = Vec::new();
        while let Some(tok) = self.peek()? && tok.ty != TokenType::RightParen {
            args.push(self.expression()?);
        }
        self.expect(&[TokenType::RightParen])?;
        self.expect(&[TokenType::Semicolon])?;
        let expr = Expression::FunctionCall {
            name,
            arguments: args,
        };
        Ok(expr)
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
                token: tok,
            };
            Ok(expr)
        }
    }

    /// \<block\> ::= {\<statement\>}
    fn block(&mut self) -> Result<Statement, ParserError> {
        let mut stats = Vec::new();
        if !self.peek_is(TokenType::RightBrace)? {
            stats.push(self.statement()?);
        }
        let stat = Statement::Block(BlockStatement{ statements: stats });
        Ok(stat)
    }
}
