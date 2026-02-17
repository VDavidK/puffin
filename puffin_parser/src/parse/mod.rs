use puffin_ast::{Ast, VarType};
use puffin_ast::span::Span;
use puffin_ast::token::{Token, TokenType};
use puffin_ast::statement::{Statement, ExpressionStatement, BreakStatement, ContinueStatement, ForStatement, IfStatement, BlockStatement, ReturnStatement};
use puffin_ast::declaration::{Declaration, VarDeclaration, Decorator, ComponentDeclaration, MethodDeclaration, SignalDeclaration, LayoutDeclaration, LayoutItemDeclaration, LayoutItemProp, LambdaFunctionBinding, BindingDeclaration, DirectBindings};
use puffin_ast::expression::{AccessorExpression, BinaryExpression, Expression, FunctionCallExpression, LiteralExpression, UnaryExpression};
use crate::lex::{PuffinLexer, LexerError};

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

/*fn is_literal(token: &Token) -> bool {
    match token.ty {
        TokenType::Identifier | TokenType::String | TokenType::Integer
        | TokenType::Float    | TokenType::KwTrue | TokenType::KwFalse => true,
        _ => false,
    }
}*/

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
    #[error("Expected literal or event binding at {0}")]
    ExpectedLiteralOrEventBindingError(Span),
    #[error("Expected literal or expression at {0}")]
    ExpectedLiteralOrExpressionError(Span),
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

    fn next_token(&mut self) -> Result<Token, ParserError> {
        self.next_token_or_none()?.ok_or(ParserError::UnexpectedEofError())
    }

    fn next_token_or_none(&mut self) -> Result<Option<Token>, ParserError> {
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
        let res = self.next_token()?;
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
        let mut decls = vec!();
        while !self.eof() {
            decls.push(self.declaration()?);
        }
        let ast = Ast::new(decls);
        dbg!(&ast);
        Ok(ast)
    }

    /// declaration ::= \<var_decl\><br>
    /// declaration ::= \<signal_decl\><br>
    /// declaration ::= \<decorated_method_decl\><br>
    /// declaration ::= \<method_decl\><br>
    /// declaration ::= \<component_decl\>
    fn declaration(&mut self) -> Result<Declaration, ParserError> {
        let pos = self.pos();
        let decl  = match self.peek()?.ok_or(ParserError::ExpectedDeclarationError(pos))?.ty {
            TokenType::KwLet | TokenType::KwConst => self.var_decl()?,
            TokenType::KwSignal => self.signal_decl()?,
            TokenType::At => self.decorated_method_decl()?,
            TokenType::KwFn => self.method_decl()?,
            TokenType::KwComponent => self.component_decl()?,
            TokenType::KwLayout => self.layout_decl()?,
            _ => return Err(ParserError::ExpectedDeclarationError(self.pos()))
        };
        Ok(decl)
    }

    /// \<component\> ::= "component", \<identifier\>, \<parameters\>, "{", {\<declaration\>}, "}"
    fn component_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwComponent])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?;
        self.expect(&[TokenType::LeftBrace])?;
        let mut decls = vec!();
        while !self.peek_is(TokenType::RightBrace)? {
            decls.push(self.declaration()?);
        }
        self.expect(&[TokenType::RightBrace])?;

        let decl = Declaration::Component(ComponentDeclaration::new(params, decls).with_name(name));
        Ok(decl)
    }

    /// \<decorated_method_decl\> ::= "@", \<identifier\>, \<parameters\>, \<method_decl\>
    fn decorated_method_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::At])?;
        let decorator_name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?;
        let mut method = self.method_decl()?;
        if let Declaration::Method(m) = &mut method {
            m.decorator = Some(Decorator::new(decorator_name, params));
            Ok(method)
        } else {
            Err(ParserError::ExpectedMethodError(self.pos().clone()))
        }
    }

    fn method_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwFn])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?;
        self.expect(&[TokenType::LeftBrace])?;
        let decl = Declaration::Method(MethodDeclaration::new(name, params, self.block_stat()?));
        self.expect(&[TokenType::RightBrace])?;
        Ok(decl)
    }

    /// \<parameters\> ::= "(", {\<identifier\>}, ")"
    fn parameters(&mut self) -> Result<Vec<Token>, ParserError> {
        let params = if self.peek_is(TokenType::LeftParen)? {
            self.next_token()?;
            let params = self.name_list()?;
            self.expect(&[TokenType::RightParen])?;
            params
        } else {
            vec!()
        };
        Ok(params)
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
        let decl = Declaration::Var(VarDeclaration::new(name, self.expression()?, ty));
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    /// \<signal\> ::= "signal", \<identifier\>, \<parameters\>, ";"
    fn signal_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwSignal])?;
        let pos = self.pos();
        let name = self.next_token_or_none()?.ok_or(ParserError::ExpectedIdentifierError(pos))?;
        let params = self.parameters()?;
        let decl = Declaration::Signal(SignalDeclaration::new(name, params));
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    /// <name_list> ::= \<identifier\>, {",", \<identifier\>}
    fn name_list(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = vec!();
        while self.peek_is(TokenType::Identifier)? {
            // Safe unwrap, as peek_is has verified the token's existence.
            names.push(self.next_token_or_none()?.unwrap().clone());
            // A comma indicates another identifier. Trailing commas are currently not allowed.
            if !self.peek_is(TokenType::Comma)? {
                break;
            } else {
                self.next_token_or_none()?;
            }
        }
        Ok(names)
    }

    /*/// \<args\> ::= \<literal\>, {",", \<literal\>}
    fn args(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = vec!();
        let pos = self.pos();
        while is_literal(self.peek()?.ok_or(ExpectedLiteralError(pos.clone()))?) {
            names.push(self.next_token()?);
            // A comma indicates another literal. Trailing commas are currently not allowed.
            if !self.peek_is(TokenType::Comma)? {
                break;
            } else {
                self.next_token_or_none()?;
            }
        }
        Ok(names)
    }*/

    /// \<statement\> ::= \<if_statement\><br>
    /// \<statement\> ::= \<expr_statement\>
    fn statement(&mut self) -> Result<Statement, ParserError> {
        let pos = self.pos();
        let stat = match self.peek()?.ok_or(ParserError::ExpectedDeclarationError(pos.clone()))?.ty {
            TokenType::KwIf => self.if_stat(),
            TokenType::KwFor => self.for_stat(),
            TokenType::KwReturn => self.return_stat(),
            TokenType::KwBreak => self.break_stat(),
            TokenType::KwThrow => self.throw_stat(),
            TokenType::KwContinue => self.continue_stat(),
            TokenType::KwMatch => self.match_stat(),
            _ => self.expr_stat()
        }?;
        Ok(stat)
    }

    fn match_stat(&mut self) -> Result<Statement, ParserError> {
        todo!();
    }

    fn break_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwBreak])?;
        self.expect(&[TokenType::Semicolon])?;
        Ok(Statement::Break(BreakStatement {}))
    }

    fn continue_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwContinue])?;
        self.expect(&[TokenType::Semicolon])?;
        Ok(Statement::Continue(ContinueStatement {}))
    }

    fn throw_stat(&mut self) -> Result<Statement, ParserError> {
        todo!();
    }

    fn for_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwFor])?;
        let var_name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::KwIn])?;
        let iterable = Box::new(self.expression()?);
        let end_range = if self.peek_is(TokenType::Colon)? {
            self.next_token()?;
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.expect(&[TokenType::LeftBrace])?;
        let block = Box::new(self.block_stat()?);
        self.expect(&[TokenType::RightBrace])?;
        let stat = Statement::For(ForStatement {
            var_name,
            iterable,
            end_range,
            block,
        });
        Ok(stat)
    }

    fn return_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwReturn])?;
        let expr: Option<Box<Expression>> = if !self.peek_is(TokenType::Semicolon)? {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        let stat = Statement::Return(ReturnStatement {
            expression: expr,
        });
        self.expect(&[TokenType::Semicolon])?;
        Ok(stat)
    }

    /// \<if_stat\> ::= "if", \<expression\>, "{", \<block\>, "}", {"else", \<if_stat}>}, \[\"else\", \<block\>\]
    fn if_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwIf])?;
        let condition = self.expression()?;
        self.expect(&[TokenType::LeftBrace])?;
        let if_block = self.block_stat()?;
        self.expect(&[TokenType::RightBrace])?;
        let else_block: Option<Box<Statement>> = match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
            TokenType::KwElse => {
                self.next_token_or_none()?;
                match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                    TokenType::KwIf => Some(Box::new(self.if_stat()?)),
                    _ => {
                        self.expect(&[TokenType::LeftBrace])?;
                        let stat = Some(Box::new(self.block_stat()?));
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
        let expr = self.binary_expr(0)?;
        Ok(expr)
    }

    fn expr_stat(&mut self) -> Result<Statement, ParserError> {
        let pos = self.pos();
        let expr = match self.peek()?.ok_or(ParserError::ExpectedStatementError(pos.clone()))?.ty {
            TokenType::Identifier => {
                let name = Expression::Literal(LiteralExpression::new(self.expect(&[TokenType::Identifier])?));
                let tok = self.expect(&[TokenType::LeftParen, TokenType::Assign])?;
                match tok.ty {
                    TokenType::LeftParen => {
                        let mut arguments= vec!();
                        while let Some(tok) = self.peek()? && tok.ty != TokenType::RightParen {
                            arguments.push(self.expression()?);
                        }
                        let expr = Expression::FunctionCall(FunctionCallExpression::new(Box::new(name), arguments));
                        self.expect(&[TokenType::RightParen])?;
                        expr
                    },
                    TokenType::Assign => Expression::Binary(BinaryExpression::new(
                        Box::new(name),
                        tok.clone(),
                        Box::new(self.expression()?))),
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

    fn binary_expr(&mut self, precedence: usize) -> Result<Expression, ParserError> {
        let mut expr: Expression = self.unary_expr()?;
        loop {
            let op = self.peek()?.ok_or(ParserError::UnexpectedEofError())?.to_owned();
            let prec = get_op_precedence(op.ty);
            if prec == 0 || prec <= precedence {
                break Ok(expr)
            }
            // Consume the peeked token
            self.next_token()?;
            if self.peek_is(TokenType::Semicolon)? {
                break Ok(expr)
            }
            let rhs = self.binary_expr(prec)?;
            expr = Expression::Binary(BinaryExpression::new(Box::new(expr), op, Box::new(rhs)));
        }
    }

    fn unary_expr(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        match self.peek()?.ok_or(ParserError::ExpectedUnaryOperatorError(pos))?.ty {
            // TODO: Increment/Decrement?
            TokenType::Plus | TokenType::Minus | TokenType::KwNot => {
                let expr = Expression::Unary(UnaryExpression::new(self.next_token()?,Box::new(self.unary_expr()?)));
                Ok(expr)
            },
            _ => self.primary_expr(),
        }
    }

    /*fn call_expression(&mut self) -> Result<Expression, ParserError> {
        let name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::LeftParen])?;
        let mut args: Vec<Expression> = vec!();
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
    }*/

    /// <primary_exp> ::= (\<literal\> | \<paren_exp\>), \[\<call_exp\>\]<br>
    /// <paren_exp> ::= "(", \<expression\>, ")"
    fn primary_expr(&mut self) -> Result<Expression, ParserError> {
        let pos = self.lexer.attach_snippet(self.pos().clone());
        let tok = self.peek()?.ok_or(ParserError::UnexpectedEofError())?;
        let mut expr = match tok.ty {
            TokenType::String | TokenType::Integer |
            TokenType::Float | TokenType::KwTrue |
            TokenType::KwFalse | TokenType::Identifier => {
                let expr = Expression::Literal(LiteralExpression::new(self.next_token()?));
                Ok(expr)
            },
            // <paren_exp>
            TokenType::LeftParen => {
                self.next_token()?;
                let expr = self.expression()?;
                self.expect(&[TokenType::RightParen])?;
                Ok(expr)
            },
            _ => Err(ParserError::ExpectedLiteralError(pos))
        }?;
        loop {
            match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                TokenType::Dot => {
                    self.next_token()?;
                    let field = self.next_token()?;
                    expr = Expression::Accessor(AccessorExpression::new(Box::new(expr), field));
                },
                TokenType::LeftParen => {
                    self.next_token()?;
                    let mut exprs = vec!();
                    if !self.peek_is(TokenType::RightParen)? {
                        exprs.push(self.expression()?);
                        while self.peek_is(TokenType::Comma)? {
                            self.next_token()?;
                            exprs.push(self.expression()?);
                        }
                    }
                    self.expect(&[TokenType::RightParen])?;
                    expr = Expression::FunctionCall(FunctionCallExpression::new(Box::new(expr), exprs));
                }
                _ => break
            }
        }
        Ok(expr)
    }

    /// \<block\> ::= {\<statement\>}
    fn block_stat(&mut self) -> Result<Statement, ParserError> {
        let mut stats = vec!();
        while !self.peek_is(TokenType::RightBrace)? {
            stats.push(self.statement()?);
        }
        let stat = Statement::Block(BlockStatement{ statements: stats });
        Ok(stat)
    }

    fn layout_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwLayout])?;
        self.expect(&[TokenType::LeftBrace])?;
        let mut components = vec!();
        while !self.peek_is(TokenType::RightBrace)? {
            components.push(self.layout_item()?);
        }
        self.expect(&[TokenType::RightBrace])?;
        let decl = Declaration::Layout(LayoutDeclaration::new(components));
        Ok(decl)
    }

    /// <layout_item> ::= \<identifier\>, {\<prop\>, "=", \<primary_expression\>}, "{", {\<layout_item\>}, "}"<br>
    /// <layout_item> ::= \<identifier\>, {\<prop\>, "=", \<primary_expression\>}, \[\<string\>\], ";"
    fn layout_item(&mut self) -> Result<Declaration, ParserError> {
        let name = self.expect(&[TokenType::Identifier])?;
        let mut bindings = vec!();
        let mut declarations = vec!();
        while self.peek_is(TokenType::Identifier)? {
            bindings.push(self.layout_binding()?);
        }
        let string_literal = if self.peek_is(TokenType::String)? || self.peek_is(TokenType::Identifier)? {
            Some(self.next_token()?)
        } else {
            None
        };
        if self.peek_is(TokenType::LeftBrace)? {
            self.next_token()?;
            while !self.peek_is(TokenType::RightBrace)? {
                declarations.push(self.layout_item()?);
            }
            self.expect(&[TokenType::RightBrace])?;
        } else {
            self.expect(&[TokenType::Semicolon])?;
        }
        let item = Declaration::LayoutItem(LayoutItemDeclaration::new(name, bindings, string_literal, declarations));
        Ok(item)
    }

    /// <expr_list> ::= \<expression\>, {",", \<expression\>}
    fn expr_list(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut exprs = vec!();
        exprs.push(self.expression()?);
        while self.peek_is(TokenType::Semicolon)? {
            self.next_token()?;
            exprs.push(self.expression()?);
        }
        Ok(exprs)
    }

    fn layout_binding(&mut self) -> Result<BindingDeclaration, ParserError> {
        let name = self.expect(&[TokenType::Identifier])?;
        if self.peek_is(TokenType::LeftParen)? {
            self.next_token()?;
            let parameters = self.name_list()?;
            self.expect(&[TokenType::RightParen])?;
            self.expect(&[TokenType::Assign])?;
            self.expect(&[TokenType::LeftBrace])?;
            let exprs = if !self.peek_is(TokenType::RightBrace)? {
                self.expr_list()?
            } else {
                vec!()
            };
            self.expect(&[TokenType::RightBrace])?;
            Ok(BindingDeclaration::new(name, LayoutItemProp::Lambda(LambdaFunctionBinding::new(parameters, exprs))))
        } else {
            self.expect(&[TokenType::Assign])?;
            match self.peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                TokenType::Identifier => {
                    let decl = BindingDeclaration::new(name, LayoutItemProp::DirectBindings(
                        DirectBindings::new(vec![self.expect(&[TokenType::Identifier])?]))
                    );
                    Ok(decl)
                },
                TokenType::LeftBracket => {
                    self.next_token()?;
                    let names = self.name_list()?;
                    self.expect(&[TokenType::RightBracket])?;
                    let decl = BindingDeclaration::new(name, LayoutItemProp::DirectBindings(DirectBindings::new(names)));
                    Ok(decl)
                },
                TokenType::LeftBrace => {
                    self.next_token()?;
                    let exprs = if !self.peek_is(TokenType::RightBrace)? {
                        self.expr_list()?
                    } else {
                        vec!()
                    };
                    self.expect(&[TokenType::RightBrace])?;
                    let decl = BindingDeclaration::new(name, LayoutItemProp::Lambda(LambdaFunctionBinding::new(vec!(), exprs )));
                    Ok(decl)
                },
                t => Err(ParserError::UnexpectedTokenError{
                    span: self.pos().to_owned(),
                    expected: vec![TokenType::Assign, TokenType::LeftBracket],
                    received: t
                })
            }
        }
    }
}
