use puffin_ast::{Ast, VarType};
use puffin_ast::span::Span;
use puffin_ast::token::{Token, TokenType};
use puffin_ast::statement::{Statement, AssignStatement, ExpressionStatement, BreakStatement, ContinueStatement, ForStatement, IfStatement, BlockStatement, ReturnStatement, MatchStatement, VariableDeclarationStatement, IncrementStatement, DecrementStatement, OpAssignStatement};
use puffin_ast::declaration::{Declaration, VarDeclaration, Decorator, ComponentDeclaration, MethodDeclaration, SignalDeclaration, LayoutDeclaration, RequireDeclaration, UseDeclaration, ExportDeclaration, EnumDeclaration};
use puffin_ast::expression::{AccessorExpression, BinaryExpression, Expression, FunctionCallExpression, LiteralExpression, UnaryExpression, ArrayExpression, DictionaryExpression, MatchExpression, IndexExpression};
use puffin_ast::markup::{Markup, LambdaFunctionBinding, MarkupBinding, DirectBindings, ComponentRender, IterativeRender, IfConditionalRender, MatchConditionalRender, LayoutRender, StyleRender};
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
    #[error("Expected one of {expected:?} found '{received}' at {span}")]
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
    #[error("Syntax error at {0}")]
    SyntaxError(Span),
}

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct PuffinParser<'a> {
    lexer: PuffinLexer<'a>,
    current_token: Option<Token>,
}

impl<'a> PuffinParser<'a> {
    pub fn new(src: &'a str, src_name: &'a str) -> Self {
        PuffinParser {
            lexer: PuffinLexer::new(src, src_name),
            current_token: None,
        }
    }

    /// Returns the position of the parser's current token if it is assigned, or a default position if it is ```None```.
    fn pos(&self) -> Span {
        self.current_token.as_ref().map(|t| self.lexer.attach_snippet(t.span.clone())).unwrap_or_default()
    }

    /// Returns an indicator of whether the lexer's tokens have been exhausted.
    fn eof(&mut self) -> bool {
        self.safe_peek().is_ok_and(|t| t.is_none())
    }

    /// Peeks the next token and returns it if it exists. Does not consume the currently active token.
    fn safe_peek(&mut self) -> Result<Option<&Token>, ParserError> {
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

    /// Peeks the next token and returns it if it exists without consuming it. If the token is None,
    /// this method will return a ParserError.
    fn peek(&mut self) -> Result<&Token, ParserError> {
        let token = self.safe_peek()?;
        match token {
            Some(tok) => Ok(tok),
            None => Err(ParserError::UnexpectedEofError()),
        }
    }

    /// Peeks the next token and returns an indicator of whether its type matches ```expected```.
    fn peek_is(&mut self, expected: TokenType) -> Result<bool, ParserError> {
        Ok(self.safe_peek()?.is_some_and(|f| f.ty == expected))
    }

    fn next_token(&mut self) -> Result<Token, ParserError> {
        self.next_token_or_none()?.ok_or(ParserError::UnexpectedEofError())
    }

    fn next_token_or_none(&mut self) -> Result<Option<Token>, ParserError> {
        self.safe_peek()?;
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
                span: self.lexer.attach_snippet(res.span.to_owned()),
                expected: types.iter().cloned().collect::<Vec<_>>(),
                received: res.ty,
            })
        }
    }

    /// Runs the parser on the source file provided when it was initialized.
    pub fn run(mut self) -> Result<Ast, ParserError> {
        let mut decls = vec![];
        while !self.eof() {
            decls.push(self.declaration()?);
        }
        let ast = Ast::new(decls);
        Ok(ast)
    }

    /// declaration ::= \<var_decl\><br>
    /// declaration ::= \<signal_decl\><br>
    /// declaration ::= \<decorated_method_decl\><br>
    /// declaration ::= \<method_decl\><br>
    /// declaration ::= \<component_decl\>
    fn declaration(&mut self) -> Result<Declaration, ParserError> {
        let decl  = match self.peek()?.ty {
            TokenType::KwLet | TokenType::KwConst => self.var_decl()?,
            TokenType::KwSignal => self.signal_decl()?,
            TokenType::At => self.decorated_method_decl()?,
            TokenType::KwFn => self.method_decl()?,
            TokenType::KwComponent => self.component_decl()?,
            TokenType::KwLayout => self.layout_decl()?,
            TokenType::KwRequire => self.require_decl()?,
            TokenType::KwUse => self.use_decl()?,
            TokenType::KwExport => self.export_decl()?,
            TokenType::KwEnum => self.enum_decl()?,
            _ => return Err(ParserError::ExpectedDeclarationError(self.pos()))
        };
        Ok(decl)
    }

    fn export_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwExport])?;
        let decl = Declaration::Export(ExportDeclaration::new(self.declaration()?));
        Ok(decl)
    }

    fn enum_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwEnum])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let mut members = vec![];
        self.expect(&[TokenType::LeftBrace])?;
        while !self.peek_is(TokenType::RightBrace)? {
            members.push(self.expect(&[TokenType::Identifier])?);
            if self.peek_is(TokenType::Comma)? {
                self.next_token()?;
            } else {
                self.expect(&[TokenType::RightBrace])?;
                break
            }
        }
        self.expect(&[TokenType::RightBrace])?;
        let decl = Declaration::Enum(EnumDeclaration::new(name, members));
        Ok(decl)
    }

    fn require_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwRequire])?;
        let decl = Declaration::Require(RequireDeclaration::new(self.expect(&[TokenType::String])?));
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    fn use_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwUse])?;
        let mut expr = Expression::Literal(LiteralExpression::new(self.expect(&[TokenType::Identifier])?));
        while !self.peek_is(TokenType::Semicolon)? {
            self.expect(&[TokenType::Dot])?;
            expr = Expression::Accessor(AccessorExpression::new(expr, self.expect(&[TokenType::Identifier])?));
        }
        self.expect(&[TokenType::Semicolon])?;
        let decl = Declaration::Use(UseDeclaration::new(expr));
        Ok(decl)
    }

    /// \<component\> ::= "component", \<identifier\>, \<parameters\>, "{", {\<declaration\>}, "}"
    fn component_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwComponent])?;
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?;
        self.expect(&[TokenType::LeftBrace])?;
        let mut decls = vec![];
        while !self.peek_is(TokenType::RightBrace)? {
            decls.push(self.declaration()?);
        }
        self.expect(&[TokenType::RightBrace])?;

        Ok(ComponentDeclaration::new(params, decls).with_name(name).into())
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
            Err(ParserError::ExpectedMethodError(self.pos()))
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
            vec![]
        };
        Ok(params)
    }

    /// \<var_decl\> ::= "const" | "let", \<identifier\>, "=", \<expression\>, ";"
    fn var_decl(&mut self) -> Result<Declaration, ParserError> {
        let ty = self.var_type()?;
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
        let name = self.expect(&[TokenType::Identifier])?;
        let params = self.parameters()?;
        let decl = Declaration::Signal(SignalDeclaration::new(name, params));
        self.expect(&[TokenType::Semicolon])?;
        Ok(decl)
    }

    /// <name_list> ::= \<identifier\>, {",", \<identifier\>}
    fn name_list(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = vec![];
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

    /// \<statement\> ::= \<if_statement\><br>
    /// \<statement\> ::= \<expr_call_or_assign_statement\>
    fn statement(&mut self) -> Result<Statement, ParserError> {
        let stat = match self.peek()?.ty {
            TokenType::KwIf => self.if_stat(),
            TokenType::KwFor => self.for_stat(),
            TokenType::KwReturn => self.return_stat(),
            TokenType::KwBreak => self.break_stat(),
            TokenType::KwThrow => self.throw_stat(),
            TokenType::KwContinue => self.continue_stat(),
            TokenType::KwMatch => self.match_stat(),
            TokenType::KwLet | TokenType::KwConst => self.var_stat(),
            _ => self.expr_or_assign_stat()
        }?;
        Ok(stat)
    }

    fn var_stat(&mut self) -> Result<Statement, ParserError> {
        let var_type = self.var_type()?;
        let name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::Assign])?;
        let expr = self.expression()?;
        self.expect(&[TokenType::Semicolon])?;
        Ok(VariableDeclarationStatement::new(name, expr, var_type).into())
    }

    fn var_type(&mut self) -> Result<VarType, ParserError> {
        match self.expect(&[TokenType::KwConst, TokenType::KwLet])?.ty {
            TokenType::KwConst => Ok(VarType::Const),
            TokenType::KwLet => Ok(VarType::Let),
            t => Err(ParserError::ExpectedVarTypeError(self.pos(), t))
        }
    }

    fn match_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwMatch])?;
        let comparator = self.expression()?;
        let mut cases = vec![];
        let mut default_case = None;
        self.expect(&[TokenType::LeftBrace])?;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.next_token()?;
                self.expect(&[TokenType::Arrow])?;
                let default_name = if self.peek_is(TokenType::Identifier)? {
                    Some(self.next_token()?)
                } else {
                    None
                };
                let stat = self.statement()?;
                default_case = Some((default_name, stat));
                break;
            } else {
                let expr = self.expression()?;
                self.expect(&[TokenType::Arrow])?;
                let stat = if self.peek_is(TokenType::LeftBrace)? {
                    self.next_token()?;
                    self.block_stat()?
                } else {
                    let expr = self.expression()?;
                    ExpressionStatement::new(expr).into()
                };
                cases.push((expr, stat));
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                }
            }
        }
        self.expect(&[TokenType::RightBrace])?;

        Ok(MatchStatement::new(comparator, cases, default_case).into())
    }

    fn break_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwBreak])?;
        self.expect(&[TokenType::Semicolon])?;
        Ok(BreakStatement.into())
    }

    fn continue_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwContinue])?;
        self.expect(&[TokenType::Semicolon])?;
        Ok(ContinueStatement.into())
    }

    fn throw_stat(&mut self) -> Result<Statement, ParserError> {
        todo!();
    }

    fn for_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwFor])?;
        let var_name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::KwIn])?;
        let iterable = self.expression()?;
        let end_range = if self.peek_is(TokenType::Colon)? {
            self.next_token()?;
            Some(self.expression()?)
        } else {
            None
        };
        self.expect(&[TokenType::LeftBrace])?;
        let block = self.block_stat()?;
        self.expect(&[TokenType::RightBrace])?;
        Ok(ForStatement::new(var_name, iterable, end_range, block).into())
    }

    fn return_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(&[TokenType::KwReturn])?;
        let expr: Option<Expression> = if !self.peek_is(TokenType::Semicolon)? {
            Some(self.expression()?)
        } else {
            None
        };
        let stat = ReturnStatement::new(expr).into();
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
        let else_block: Option<Statement> = match self.safe_peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
            TokenType::KwElse => {
                self.next_token_or_none()?;
                match self.safe_peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                    TokenType::KwIf => Some(self.if_stat()?),
                    _ => {
                        self.expect(&[TokenType::LeftBrace])?;
                        let stat = Some(self.block_stat()?);
                        self.expect(&[TokenType::RightBrace])?;
                        stat
                    }
                }
            },
            _ => None,
        };
        Ok(IfStatement::new(condition, if_block, else_block).into())
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        match self.peek()?.ty {
            TokenType::KwMatch => self.match_expr(),
            _ => self.binary_expr(0),
        }
    }

    fn match_expr(&mut self) -> Result<Expression, ParserError> {
        self.expect(&[TokenType::KwMatch])?;
        let comparator = self.expression()?;
        let mut cases = vec![];
        self.expect(&[TokenType::LeftBrace])?;
        let mut default_case = None;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.next_token()?;
                self.expect(&[TokenType::Arrow])?;
                let default_name = if self.peek_is(TokenType::Identifier)? {
                    Some(self.next_token()?)
                } else {
                    None
                };
                let expr = self.expression()?;
                default_case = Some((default_name, expr));
                break;
            } else {
                let lhs = self.expression()?;
                self.expect(&[TokenType::Arrow])?;
                let rhs = self.expression()?;
                cases.push((lhs, rhs));
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                }
            }
        }
        self.expect(&[TokenType::RightBrace])?;
        Ok(MatchExpression::new(comparator, cases, default_case).into())
    }

    fn expr_or_assign_stat(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        match self.peek()?.ty {
            TokenType::Assign => {
                self.next_token()?;
                let rhs = self.expression()?;
                self.expect(&[TokenType::Semicolon])?;
                Ok(AssignStatement::new(expr, rhs).into())
            },
            TokenType::Semicolon => {
                self.next_token()?;
                Ok(ExpressionStatement::new(expr).into())
            },
            TokenType::Increment => {
                self.next_token()?;
                self.expect(&[TokenType::Semicolon])?;
                Ok(IncrementStatement::new(expr).into())
            },
            TokenType::Decrement => {
                self.next_token()?;
                self.expect(&[TokenType::Semicolon])?;
                Ok(DecrementStatement::new(expr).into())
            },
            TokenType::IncrementAssign
            | TokenType::DecrementAssign
            | TokenType::MulAssign
            | TokenType::DivAssign =>  {
                let op = self.next_token()?;
                let rhs = self.expression()?;
                self.expect(&[TokenType::Semicolon])?;
                Ok(OpAssignStatement::new(expr, op, rhs).into())
            },
            _ => Err(ParserError::ExpectedStatementError(self.pos()))
        }
    }

    fn binary_expr(&mut self, precedence: usize) -> Result<Expression, ParserError> {
        let mut expr: Expression = self.unary_expr()?;
        loop {
            let op = self.safe_peek()?.ok_or(ParserError::UnexpectedEofError())?.to_owned();
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
            expr = Expression::Binary(BinaryExpression::new(expr, op, rhs));
        }
    }

    fn unary_expr(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        match self.safe_peek()?.ok_or(ParserError::ExpectedUnaryOperatorError(pos))?.ty {
            // TODO: Increment/Decrement?
            TokenType::Plus | TokenType::Minus | TokenType::KwNot => {
                let expr = Expression::Unary(UnaryExpression::new(self.next_token()?, self.unary_expr()?));
                Ok(expr)
            },
            _ => self.primary_expr(),
        }
    }

    /// <primary_exp> ::= (\<literal\> | \<paren_exp\>), \[\<call_exp\>\]<br>
    /// <paren_exp> ::= "(", \<expression\>, ")"
    fn primary_expr(&mut self) -> Result<Expression, ParserError> {
        let pos = self.lexer.attach_snippet(self.pos().clone());
        let tok = self.safe_peek()?.ok_or(ParserError::UnexpectedEofError())?;
        let mut expr = match tok.ty {
            TokenType::String | TokenType::Integer | TokenType::Float
            | TokenType::KwTrue | TokenType::KwFalse | TokenType::Identifier
            | TokenType::KwNull => {
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
            TokenType::LeftBracket => {
                // Array
                self.next_token()?;
                let array = if !self.peek_is(TokenType::RightBracket)? {
                self.expr_list(TokenType::RightBracket, TokenType::Comma)?
                } else {
                    vec![]
                };
                self.expect(&[TokenType::RightBracket])?;
                let expr = Expression::Array(ArrayExpression::new(array));
                return Ok(expr);
            },
            TokenType::LeftBrace => {
                // Dictionary
                todo!();
            }
            _ => Err(ParserError::ExpectedLiteralError(pos))
        }?;
        loop {
            match self.safe_peek()?.ok_or(ParserError::UnexpectedEofError())?.ty {
                TokenType::Dot => {
                    self.next_token()?;
                    let field = self.next_token()?;
                    expr = Expression::Accessor(AccessorExpression::new(expr, field));
                },
                TokenType::LeftParen => {
                    self.next_token()?;
                    let mut exprs = vec![];
                    if !self.peek_is(TokenType::RightParen)? {
                        exprs.push(self.expression()?);
                        while self.peek_is(TokenType::Comma)? {
                            self.next_token()?;
                            exprs.push(self.expression()?);
                        }
                    }
                    self.expect(&[TokenType::RightParen])?;
                    expr = FunctionCallExpression::new(expr, exprs).into();
                },
                TokenType::LeftBracket => {
                    self.next_token()?;
                    let indexer = self.expression()?;
                    self.expect(&[TokenType::RightBracket])?;
                    expr = IndexExpression::new(indexer, expr).into();
                },
                _ => break
            }
        }
        Ok(expr)
    }

    /// <expr_list> ::= \<expression\>, {",", \<expression\>}
    fn expr_list(&mut self, delimiter: TokenType, separator: TokenType) -> Result<Vec<Expression>, ParserError> {
        let mut exprs = vec![];
        exprs.push(self.expression()?);
        while self.peek_is(separator)? && !self.peek_is(delimiter)? {
            self.next_token()?;
            exprs.push(self.expression()?);
        }
        Ok(exprs)
    }

    /// \<block\> ::= {\<statement\>}
    fn block_stat(&mut self) -> Result<Statement, ParserError> {
        let mut stats = vec![];
        while !self.peek_is(TokenType::RightBrace)? {
            stats.push(self.statement()?);
        }
        let stat = Statement::Block(BlockStatement{ statements: stats });
        Ok(stat)
    }

    fn layout_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(&[TokenType::KwLayout])?;
        let mut parameters = vec![];
        let name = if self.peek_is(TokenType::Identifier)? {
            let name = self.next_token()?;
            if self.peek_is(TokenType::LeftParen)? {
                self.next_token()?;
                loop {
                    parameters.push(self.expect(&[TokenType::Identifier])?);
                    if self.peek_is(TokenType::Comma)? {
                        self.next_token()?;
                    }
                    if self.peek_is(TokenType::RightParen)? {
                        self.next_token()?;
                        break;
                    }
                }
            }
            Some(name)
        } else {
            None
        };
        self.expect(&[TokenType::LeftBrace])?;
        let markup = self.markup()?;
        self.expect(&[TokenType::RightBrace])?;
        Ok(LayoutDeclaration::new(markup, parameters).with_name(name).into())
    }

    fn markup(&mut self) -> Result<Vec<Markup>, ParserError> {
        let mut markup = vec![];
        loop {
            let markup_item = match self.peek()?.ty {
                TokenType::KwMatch => self.match_markup()?,
                TokenType::KwIf => self.if_markup()?,
                TokenType::KwFor => self.for_markup()?,
                TokenType::Identifier => {
                    self.markup_item()?
                },
                TokenType::KwStyle => {
                    self.next_token()?;
                    let mut style_rules = vec![];
                    if self.peek_is(TokenType::LeftBrace)? {
                        self.next_token()?;
                        while !self.peek_is(TokenType::RightBrace)? {
                            style_rules.push(self.style_markup()?);
                        }
                        self.expect(&[TokenType::RightBrace])?;
                    } else if self.peek_is(TokenType::Identifier)? {
                        style_rules.push(self.style_markup()?);
                    }
                    StyleRender::new(style_rules).into()
                },
                _ => break
            };
            markup.push(markup_item);
        }
        Ok(markup)
    }

    fn style_markup(&mut self) -> Result<(Token, Expression), ParserError> {
        let style_name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::Assign]);
        let value = self.expression()?;
        self.expect(&[TokenType::Semicolon]);
        Ok((style_name, value))
    }

    fn match_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(&[TokenType::KwMatch])?;
        let comparator = self.expression()?;
        self.expect(&[TokenType::LeftBrace])?;
        let mut cases = vec![];
        let mut default_case = None;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.expect(&[TokenType::KwDefault])?;
                let token = if self.peek_is(TokenType::Identifier)? {
                    Some(self.next_token()?)
                } else {
                    None
                };
                self.expect(&[TokenType::Arrow])?;
                let markup = self.markup()?;
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                }
                default_case = Some((token, markup));
                break;
            } else {
                let lhs = self.expression()?;
                self.expect(&[TokenType::Arrow])?;
                let rhs = if self.peek_is(TokenType::LeftBrace)? {
                    self.markup()?
                } else {
                    vec![self.markup_item()?]
                };
                if !self.peek_is(TokenType::RightBrace)? {
                    self.expect(&[TokenType::Comma])?;
                }
                cases.push((lhs, rhs));
            }
        }
        self.expect(&[TokenType::RightBrace])?;
        Ok(MatchConditionalRender::new(comparator, cases, default_case).into())
    }

    fn if_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(&[TokenType::KwIf])?;
        let condition = self.expression()?;
        let if_markup = self.markup_block()?;
        let mut elseif_markup = None;
        let mut else_markup = vec![];
        if self.peek_is(TokenType::KwElse)? {
        self.next_token()?;
            if self.peek_is(TokenType::KwIf)? {
                elseif_markup = Some(self.if_markup()?);
            } else {
                else_markup = self.markup_block()?
            }
        }
        Ok(IfConditionalRender::new(condition, if_markup, elseif_markup, else_markup).into())
    }

    fn for_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(&[TokenType::KwFor])?;
        let name = self.expect(&[TokenType::Identifier])?;
        self.expect(&[TokenType::KwIn])?;
        let iterable = self.expression()?;
        let end_range = if self.peek_is(TokenType::Colon)? {
            self.next_token()?;
            Some(self.expression()?)
        } else {
            None
        };
        let block = self.markup_block()?;
        Ok(IterativeRender::new(name, iterable, end_range, block).into())
    }

    fn markup_item(&mut self) -> Result<Markup, ParserError> {
        let name = self.next_token()?;
        if self.peek_is(TokenType::LeftParen)? {
            self.next_token()?;
            let mut exprs = vec![];
            while !self.peek_is(TokenType::RightParen)? {
                exprs.push(self.expression()?);
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                } else {
                    break;
                }
            }
            self.expect(&[TokenType::RightParen])?;
            self.expect(&[TokenType::Semicolon])?;
            return Ok(LayoutRender::new(name, exprs).into())
        }
        let bindings = if self.peek_is(TokenType::Identifier)? {
            let mut bindings = vec![];
            while self.peek_is(TokenType::Identifier)? {
                let name = self.next_token()?;
                let mut is_lambda = false;
                let parameters = if self.peek_is(TokenType::LeftParen)? {
                    is_lambda = true;
                    self.expect(&[TokenType::LeftParen])?;
                    let mut parameters = vec![];
                    while !self.peek_is(TokenType::RightParen)? {
                        parameters.push(self.expect(&[TokenType::Identifier])?);
                        if self.peek_is(TokenType::Comma)? {
                            self.expect(&[TokenType::Comma])?;
                        } else {
                            self.expect(&[TokenType::RightParen])?;
                            break;
                        }
                    }
                    parameters
                } else {
                    vec![]
                };
                self.expect(&[TokenType::Assign])?;
                if self.peek_is(TokenType::LeftBrace)? {
                    is_lambda = true;
                }
                let binding = if is_lambda {
                    self.expect(&[TokenType::LeftBrace])?;
                    let exprs = self.expr_list(TokenType::RightBrace, TokenType::Semicolon)?;
                    self.expect(&[TokenType::RightBrace])?;
                    LambdaFunctionBinding::new(parameters, exprs).into()
                } else {
                    let tokens = if self.peek_is(TokenType::LeftBracket)? {
                        let mut tokens = vec![];
                        self.next_token()?;
                        while !self.peek_is(TokenType::RightBracket)? {
                            tokens.push(self.expect(&[TokenType::Identifier])?);
                            if self.peek_is(TokenType::Comma)? {
                                self.next_token()?;
                            } else {
                                break;
                            }
                        }
                        self.expect(&[TokenType::RightBracket])?;
                        tokens
                    } else {
                        vec![self.expect(&[TokenType::Identifier])?]
                    };
                    DirectBindings::new(tokens).into()
                };
                bindings.push(MarkupBinding::new(name, binding))
            }
            bindings
        } else {
            vec![]
        };
        let string_literal = if self.peek_is(TokenType::String)? {
            Some(self.next_token()?)
        } else {
            None
        };
        let children = if string_literal.is_some() {
            self.expect(&[TokenType::Semicolon])?;
            vec![]
        } else if self.peek_is(TokenType::LeftBrace)? {
            self.markup_block()?
        } else {
            vec![]
        };
        Ok(ComponentRender::new(name, bindings, string_literal, children).into())
    }

    fn markup_block(&mut self) -> Result<Vec<Markup>, ParserError> {
        self.expect(&[TokenType::LeftBrace])?;
        let mut markup = self.markup()?;
        self.expect(&[TokenType::RightBrace])?;
        Ok(markup)
    }
}
