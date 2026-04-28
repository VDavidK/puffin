use puffin_ast::{Ast, VarType};
use puffin_ast::snippet::{IntoSnippet, Snippet};
use puffin_ast::span::Span;
use puffin_ast::token::{Token, TokenType};
use puffin_ast::statement::{Statement, AssignStatement, ExpressionStatement, BreakStatement, ContinueStatement, ForStatement, IfStatement, BlockStatement, ReturnStatement, MatchStatement, VariableDeclarationStatement, IncrementStatement, DecrementStatement, OpAssignStatement, ThrowStatement, CatchStatement, RaiseStatement};
use puffin_ast::declaration::{Declaration, VarDeclaration, Decorator, ComponentDeclaration, MethodDeclaration, SignalDeclaration, LayoutDeclaration, RequireDeclaration, UseDeclaration, EnumDeclaration, ErrorDeclaration, ConstructorDeclaration};
use puffin_ast::expression::{AccessorExpression, BinaryExpression, Expression, FunctionCallExpression, LiteralExpression, UnaryExpression, ArrayExpression, DictionaryExpression, MatchExpression, IndexExpression};
use puffin_ast::expression::Expression::FunctionCall;
use puffin_ast::markup::{Markup, LambdaFunctionBinding, MarkupBinding, DirectBindings, ComponentRender, IterativeRender, IfConditionalRender, MatchConditionalRender, LayoutRender, StyleRender, MarkupBlock};
use crate::lex::{PuffinLexer, LexerError};
use crate::parse::ParserError::{DuplicateConstructor, InvalidExport, MissingComponentFileName};

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

#[inline]
fn get_option_inner_type(opt: &Option<TokenType>) -> String {
    opt.map_or_else(
        || "None".to_owned(),
        |f| f.to_string()
    ).to_owned()
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error(transparent)]
    FileNotFound(#[from] std::io::Error),
    #[error(transparent)]
    LexerError(#[from] LexerError),
    #[error("Expected binary operator at {0}")]
    ExpectedBinaryOperator(Box<Snippet>),
    #[error("Expected literal, received {:?} at {}", get_option_inner_type(.1), .0)]
    ExpectedLiteral(Box<Snippet>, Option<TokenType>),
    #[error("Expected literal or parenthesis expression at {0}")]
    ExpectedLiteralOrParen(Box<Snippet>),
    #[error("Expected unary operator at {0}")]
    ExpectedUnaryOperator(Box<Snippet>),
    #[error("Expected {0:?} found '{1}' at {2}")]
    UnexpectedToken(TokenType, TokenType, Box<Snippet>),
    #[error("Expected one of {expected:?} found '{received}' at {snippet}")]
    UnexpectedOneOfToken {
        snippet: Box<Snippet>,
        expected: Vec<TokenType>,
        received: TokenType,
    },
    #[error("Unexpected end of file")]
    UnexpectedEof(),
    #[error("Expected declaration at {0}")]
    ExpectedDeclaration(Box<Snippet>),
    #[error("Expected identifier at {0}")]
    ExpectedIdentifier(Box<Snippet>),
    #[error("Expected statement at {0}")]
    ExpectedStatement(Box<Snippet>),
    #[error("Expected method declaration at {0}")]
    ExpectedMethod(Box<Snippet>),
    #[error("Expected let/const at {0}, received {1}")]
    ExpectedVarType(Box<Snippet>, TokenType),
    #[error("Expected literal or event binding at {0}")]
    ExpectedLiteralOrEventBinding(Box<Snippet>),
    #[error("Expected literal or expression at {0}")]
    ExpectedLiteralOrExpression(Box<Snippet>),
    #[error("Syntax error at {0}")]
    SyntaxError(Box<Snippet>),
    #[error("Duplicate constructor declaration at {0}")]
    DuplicateConstructor(Box<Snippet>),
    #[error("Missing component file name")]
    MissingComponentFileName(),
    #[error("Invalid export at {0}, {1} cannot be exported")]
    InvalidExport(Box<Snippet>, TokenType),
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
        self.current_token.as_ref().map(|t| t.span).unwrap_or_default()
    }

    /// Returns an indicator of whether the lexer's tokens have been exhausted.
    fn eof(&mut self) -> bool {
        self.safe_peek().is_ok_and(|t| t.is_none())
    }

    /// Peeks the next token and returns it if it exists. Does not consume the currently active token.
    fn safe_peek(&mut self) -> Result<Option<&Token>, ParserError> {
        match self.current_token.as_ref() {
            Some(token) => Ok(self.current_token.as_ref()),
            None => {
                match self.lexer.next() {
                    Some(t) => {
                        self.current_token = Some(t?);
                        Ok(self.current_token.as_ref())
                    },
                    None => Ok(None),
                }
            }
        }
    }

    /// Peeks the next token and returns it if it exists without consuming it. If the token is None,
    /// this method will return a ParserError.
    fn peek(&mut self) -> Result<&Token, ParserError> {
        match self.safe_peek()? {
            Some(tok) => Ok(tok),
            None => Err(ParserError::UnexpectedEof()),
        }
    }

    /// Peeks the next token and returns an indicator of whether its type matches ```expected```.
    fn peek_is(&mut self, expected: TokenType) -> Result<bool, ParserError> {
        Ok(self.safe_peek()?.is_some_and(|f| f.ty == expected))
    }

    fn next_token(&mut self) -> Result<Token, ParserError> {
        self.next_token_or_none()?.ok_or(ParserError::UnexpectedEof())
    }

    fn next_token_or_none(&mut self) -> Result<Option<Token>, ParserError> {
        match self.safe_peek()? {
            Some(token) => {
                let tok = self.current_token.take().ok_or(ParserError::UnexpectedEof())?;
                self.current_token = None;
                Ok(Some(tok))
            },
            None => Ok(None),
        }
    }

    pub fn expect(&mut self, ty: TokenType) -> Result<Token, ParserError> {
        let res = self.next_token()?;
        if res.ty == ty {
            Ok(res)
        } else {
            Err(ParserError::UnexpectedToken(
                ty,
                res.ty,
                self.get_lex_snippet(),
            ))
        }
    }

    /// Fetches the next token and errors if the token is ```None``` or if
    /// its type is not in ```types```, returning it otherwise.
    pub fn expect_one_of(&mut self, types: &[TokenType]) -> Result<Token, ParserError> {
        let res = self.next_token()?;
        if types.contains(&res.ty) {
            Ok(res)
        } else {
            Err(ParserError::UnexpectedOneOfToken {
                snippet: self.get_lex_snippet(),
                expected: types.to_vec(),
                received: res.ty,
            })
        }
    }

    fn get_lex_snippet(&self) -> Box<Snippet> {
        let span = match &self.current_token {
            Some(tok) => tok.span,
            None => self.lexer
                .get_span()
        };
        let snippet = span
            .into_snippet(
                self.lexer.get_src_ref(),
                self.lexer.get_src_name(),
                1
        );
        Box::new(snippet)
    }

    /// Consumes tokens while they match one of the token types provided in `expected`,
    /// terminating the process if the token matches `terminator`, skipping the token that follows `expected` if it matches `break_skip`
    /// and terminating if it does not.
    fn consume_while_not(&mut self, expected: &[TokenType], terminator: TokenType, break_skip: TokenType) -> Result<Vec<Token>, ParserError> {
        let mut tokens = vec![];
        while !self.peek_is(terminator)? {
            tokens.push(self.expect_one_of(expected)?);
            if self.peek_is(break_skip)? {
                self.next_token()?;
            } else {
                break;
            }
        }
        Ok(tokens)
    }

    /// Runs the parser on the source file provided when it was initialized.
    pub(crate) fn run(mut self) -> Result<Ast, ParserError> {
        let component_name = self.lexer.get_src_name().ok_or(MissingComponentFileName())?;
        let mut ast = Ast::new(component_name);
        while !self.eof() {
            ast.add_decl(self.declaration()?);
        }
        Ok(ast)
    }

    fn declaration(&mut self) -> Result<Declaration, ParserError> {
        let decl  = match self.peek()?.ty {
            TokenType::KwLet | TokenType::KwConst => self.var_decl(false),
            TokenType::KwSignal => self.signal_decl(),
            TokenType::At => self.decorated_method_decl(),
            TokenType::KwFn => self.method_decl(false, None),
            TokenType::KwComponent => self.component_decl(),
            TokenType::KwNew => self.constructor_decl(),
            TokenType::KwLayout => self.layout_decl(),
            TokenType::KwRequire => self.require_decl(),
            TokenType::KwUse => self.use_decl(),
            TokenType::KwExport => self.export_decl(),
            TokenType::KwEnum => self.enum_decl(false),
            TokenType::KwError => self.error_decl(),
            _ => return Err(ParserError::ExpectedDeclaration(self.get_lex_snippet()))
        }?;
        Ok(decl)
    }

    fn decorated_method_decl(&mut self) -> Result<Declaration, ParserError> {
        let decorator = self.decorator()?;
        let exported = match self.peek()?.ty {
            TokenType::KwExport => {
                self.next_token()?;
                true
            },
            _ => false,
        };
        self.method_decl(exported, Some(decorator))
    }

    fn error_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwError)?;
        self.expect(TokenType::LeftBrace)?;
        let members = self.consume_while_not(
            &[TokenType::Identifier],
            TokenType::RightBrace,
            TokenType::Comma
        )?;
        self.expect(TokenType::RightBrace)?;
        Ok(ErrorDeclaration::new(members).into())
    }

    fn export_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwExport)?;
        let decl = match self.peek()?.ty {
            TokenType::KwConst | TokenType::KwLet => self.var_decl(true)?,
            TokenType::KwFn => self.method_decl(true, None)?,
            TokenType::KwEnum => self.enum_decl(true)?,
            t => return Err(InvalidExport(self.get_lex_snippet(), t)),
        };
        Ok(decl.into())
    }

    fn enum_decl(&mut self, exported: bool) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwEnum)?;
        let name = self.expect(TokenType::Identifier)?;
        self.expect(TokenType::LeftBrace)?;
        let members = self.consume_while_not(&[TokenType::Identifier], TokenType::RightBrace, TokenType::Comma)?;
        self.expect(TokenType::RightBrace)?;
        Ok(EnumDeclaration::new(name, members, exported).into())
    }

    fn require_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwRequire)?;
        let module_name = self.expect(TokenType::String)?;
        self.expect(TokenType::Semicolon)?;
        Ok(RequireDeclaration::new(module_name).into())
    }

    fn use_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwUse)?;
        let mut expr = LiteralExpression::new(self.expect(TokenType::Identifier)?).into();
        while !self.peek_is(TokenType::Semicolon)? {
            self.expect(TokenType::Dot)?;
            expr = AccessorExpression::new(expr, self.expect(TokenType::Identifier)?).into();
        }
        self.expect(TokenType::Semicolon)?;
        Ok(UseDeclaration::new(expr).into())
    }

    fn component_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwComponent)?;
        let name = self.expect(TokenType::Identifier)?;
        let params = self.parameters()?;
        self.expect(TokenType::LeftBrace)?;
        let mut decls = vec![];
        let mut constructor = None;
        while !self.peek_is(TokenType::RightBrace)? {
            let decl = self.declaration()?;
            match decl {
                Declaration::Constructor(_) => constructor = Some(decl),
                _ => decls.push(decl),
            };
        }
        self.expect(TokenType::RightBrace)?;

        Ok(ComponentDeclaration::new(name, decls).into())
    }

    fn constructor_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwNew)?;
        let params = self.parameters()?;
        let block = self.block_stat()?;
        Ok(ConstructorDeclaration::new(params, block).into())
    }

    fn decorator(&mut self) -> Result<Decorator, ParserError> {
        self.expect(TokenType::At)?;
        let decorator_name = self.expect(TokenType::Identifier)?;
        let params = self.parameters()?;
        Ok(Decorator::new(decorator_name, params))
    }

    fn method_decl(&mut self, exported: bool, decorator: Option<Decorator>) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwFn)?;
        let name = self.expect(TokenType::Identifier)?;
        let params = self.parameters()?;
        let block = self.block_stat()?;
        Ok(MethodDeclaration::new(name, params, block).into())
    }

    fn parameters(&mut self) -> Result<Vec<Token>, ParserError> {
        let params = if self.peek_is(TokenType::LeftParen)? {
            self.next_token()?;
            let params = self.name_list()?;
            self.expect(TokenType::RightParen)?;
            params
        } else {
            vec![]
        };
        Ok(params)
    }

    fn var_decl(&mut self, exported: bool) -> Result<Declaration, ParserError> {
        let ty = self.var_type()?;
        let name = self
            .expect(TokenType::Identifier)?
            .clone();
        self.expect(TokenType::Assign)?;
        let decl = match ty {
            VarType::Let => VarDeclaration::new_let(name, self.expression()?, exported),
            VarType::Const => VarDeclaration::new_const(name, self.expression()?, exported),
        }.into();
        self.expect(TokenType::Semicolon)?;
        Ok(decl)
    }

    fn signal_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwSignal)?;
        let pos = self.pos();
        let name = self.expect(TokenType::Identifier)?;
        let params = self.parameters()?;
        let decl = SignalDeclaration::new(name, params).into();
        self.expect(TokenType::Semicolon)?;
        Ok(decl)
    }

    fn name_list(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut names: Vec<Token> = vec![];
        while self.peek_is(TokenType::Identifier)? {
            names.push(self.next_token()?);
            // A comma indicates another identifier. Trailing commas are currently not allowed.
            if !self.peek_is(TokenType::Comma)? {
                break;
            } else {
                self.next_token_or_none()?;
            }
        }
        Ok(names)
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        let stat = match self.peek()?.ty {
            TokenType::KwIf => self.if_stat(),
            TokenType::KwFor => self.for_stat(),
            TokenType::KwReturn => {
                let stat = self.return_stat()?;
                self.expect(TokenType::Semicolon)?;
                Ok(stat)
            },
            TokenType::KwBreak => self.break_stat(),
            TokenType::KwThrow => self.throw_stat(),
            TokenType::KwContinue => self.continue_stat(),
            TokenType::KwRaise => self.raise_stat(),
            TokenType::KwMatch => self.match_stat(),
            TokenType::KwLet | TokenType::KwConst => self.var_stat(),
            _ => {
                let stat = self.expr_or_assign_stat()?;
                if !TryInto::<&ExpressionStatement>::try_into(&stat).is_ok_and(|s| s.catch_block.is_some()) {
                    self.expect(TokenType::Semicolon)?;
                }
                Ok(stat)
            }
        }?;
        Ok(stat)
    }

    fn var_stat(&mut self) -> Result<Statement, ParserError> {
        let var_type = self.var_type()?;
        let name = self.expect(TokenType::Identifier)?;
        self.expect(TokenType::Assign)?;
        let expr = self.expression()?;
        let catch_block = if self.peek_is(TokenType::KwCatch)? {
            Some(self.catch_stat()?)
        } else {
            None
        };
        self.expect(TokenType::Semicolon)?;
        let mut stat = VariableDeclarationStatement::new(name, expr, var_type);
        if let Some(c) = catch_block {
            stat = stat.with_catch(c);
        }
        Ok(stat.into())
    }

    fn var_type(&mut self) -> Result<VarType, ParserError> {
        match self.expect_one_of(&[TokenType::KwConst, TokenType::KwLet])?.ty {
            TokenType::KwConst => Ok(VarType::Const),
            TokenType::KwLet => Ok(VarType::Let),
            t => Err(ParserError::ExpectedVarType(self.get_lex_snippet(), t))
        }
    }

    fn match_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwMatch)?;
        let comparator = self.expression()?;
        let mut cases = vec![];
        let mut default_case = None;
        self.expect(TokenType::LeftBrace)?;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.next_token()?;
                let default_name = if self.peek_is(TokenType::Identifier)? {
                    Some(self.next_token()?)
                } else {
                    None
                };
                self.expect(TokenType::Arrow)?;
                let stat = self.statement()?;
                default_case = Some((default_name, stat));
                break;
            } else {
                let expr = self.expression()?;
                self.expect(TokenType::Arrow)?;
                let stat = if self.peek_is(TokenType::LeftBrace)? {
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
        self.expect(TokenType::RightBrace)?;

        Ok(MatchStatement::new(comparator, cases, default_case).into())
    }

    fn raise_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwRaise)?;
        Ok(RaiseStatement.into())
    }

    fn break_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwBreak)?;
        self.expect(TokenType::Semicolon)?;
        Ok(BreakStatement.into())
    }

    fn continue_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwContinue)?;
        self.expect(TokenType::Semicolon)?;
        Ok(ContinueStatement.into())
    }

    fn catch_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwCatch)?;
        self.expect(TokenType::LeftBrace)?;
        let mut default = None;
        let mut cases = vec![];
        while !self.peek_is(TokenType::RightBrace)? {
            match self.peek()?.ty {
                TokenType::Identifier => {
                    let mut lhs = LiteralExpression::new(self.expect(TokenType::Identifier)?).into();
                    while self.peek_is(TokenType::Dot)? {
                        lhs = self.accessor_expr(lhs)?;
                    }
                    self.expect(TokenType::Arrow)?;
                    let rhs = match self.peek()?.ty {
                        TokenType::LeftBrace => {
                        self.block_stat()?
                        },
                        TokenType::KwReturn => self.return_stat()?,
                        TokenType::KwRaise => self.raise_stat()?,
                        _ => ExpressionStatement::new(self.expression()?).into()
                    };
                    cases.push((lhs, rhs));
                    if self.peek_is(TokenType::Comma)? {
                        self.next_token()?;
                    }
                },
                TokenType::KwDefault => {
                    self.next_token()?;
                    let default_name = if self.peek_is(TokenType::Identifier)? {
                        Some(self.next_token()?)
                    } else {
                        None
                    };
                    self.expect(TokenType::Arrow)?;
                    let rhs = if self.peek_is(TokenType::LeftBrace)? {
                        self.block_stat()?
                    } else {
                        match self.peek()?.ty {
                            TokenType::KwReturn => self.return_stat()?,
                            TokenType::KwRaise => self.raise_stat()?,
                            _ => ExpressionStatement::new(self.expression()?).into()
                        }
                    };
                    default = Some((default_name, rhs));
                    if self.peek_is(TokenType::Comma)? {
                        self.next_token()?;
                    }
                    break;
                },
                _ => return Err(ParserError::ExpectedStatement(self.get_lex_snippet())),
            }
        }
        self.expect(TokenType::RightBrace)?;
        let mut stat = CatchStatement::new(cases);
        if let Some(d) = default {
            stat = stat.with_default(d)
        }
        Ok(stat.into())
    }

    fn throw_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwThrow)?;
        let expr = self.expression()?;
        self.expect(TokenType::Semicolon)?;
        Ok(ThrowStatement::new(expr).into())
    }

    fn for_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwFor)?;
        let var_name = self.expect(TokenType::Identifier)?;
        self.expect(TokenType::KwIn)?;
        let iterable = self.expression()?;
        let end_range = if self.peek_is(TokenType::Colon)? {
            self.next_token()?;
            Some(self.expression()?)
        } else {
            None
        };
        let block = self.block_stat()?;
        Ok(ForStatement::new(var_name, iterable, end_range, block).into())
    }

    fn return_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwReturn)?;
        let expr: Option<Expression> = if !self.peek_is(TokenType::Semicolon)? && !self.peek_is(TokenType::Comma)? {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(ReturnStatement::new(expr).into())
    }

    fn if_stat(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::KwIf)?;
        let condition = self.expression()?;
        let if_block = self.block_stat()?;
        let else_block: Option<Statement> = match self.peek()?.ty {
            TokenType::KwElse => {
                self.next_token_or_none()?;
                match self.peek()?.ty {
                    TokenType::KwIf => Some(self.if_stat()?),
                    _ => {
                        Some(self.block_stat()?)
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
        self.expect(TokenType::KwMatch)?;
        let comparator = self.expression()?;
        let mut cases = vec![];
        self.expect(TokenType::LeftBrace)?;
        let mut default_case = None;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.next_token()?;
                self.expect(TokenType::Arrow)?;
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
                self.expect(TokenType::Arrow)?;
                let rhs = self.expression()?;
                cases.push((lhs, rhs));
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                }
            }
        }
        self.expect(TokenType::RightBrace)?;
        Ok(MatchExpression::new(comparator, cases, default_case).into())
    }

    fn expr_or_assign_stat(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        match self.peek()?.ty {
            TokenType::Assign => {
                self.next_token()?;
                let rhs = self.expression()?;
                let catch_block = if self.peek_is(TokenType::KwCatch)? {
                    let stat = self.catch_stat()?;
                    Some(stat)
                } else {
                    None
                };
                let mut stat = AssignStatement::new(expr, rhs);
                if let Some(c) = catch_block {
                    stat = stat.with_catch(c);
                }
                Ok(stat.into())
            },
            TokenType::KwCatch => {
                let catch_block = if self.peek_is(TokenType::KwCatch)? {
                    Some(self.catch_stat()?)
                } else {
                    None
                };
                let mut stat = ExpressionStatement::new(expr);
                if let Some(c) = catch_block {
                    stat = stat.with_catch(c);
                }
                Ok(stat.into())
            },
            TokenType::Semicolon => {
                Ok(ExpressionStatement::new(expr).into())
            },
            TokenType::Increment => {
                self.next_token()?;
                Ok(IncrementStatement::new(expr).into())
            },
            TokenType::Decrement => {
                self.next_token()?;
                Ok(DecrementStatement::new(expr).into())
            },
            TokenType::IncrementAssign
            | TokenType::DecrementAssign
            | TokenType::MulAssign
            | TokenType::DivAssign =>  {
                let op = self.next_token()?;
                let rhs = self.expression()?;
                Ok(OpAssignStatement::new(expr, op, rhs).into())
            },
            _ => Err(ParserError::ExpectedStatement(self.get_lex_snippet()))
        }
    }

    fn binary_expr(&mut self, precedence: usize) -> Result<Expression, ParserError> {
        let mut expr: Expression = self.unary_expr()?;
        loop {
            let op = self.peek()?.to_owned();
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
            expr = BinaryExpression::new(expr, op, rhs).into();
        }
    }

    fn unary_expr(&mut self) -> Result<Expression, ParserError> {
        let pos = self.pos();
        let snippet = self.get_lex_snippet();
        match self.safe_peek()?.ok_or(ParserError::ExpectedUnaryOperator(snippet))?.ty {
            TokenType::Plus | TokenType::Minus | TokenType::KwNot => {
                Ok(UnaryExpression::new(self.next_token()?, self.unary_expr()?).into())
            },
            _ => self.primary_expr(),
        }
    }

    fn primary_expr(&mut self) -> Result<Expression, ParserError> {
        let tok = self.peek()?;
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
                self.expect(TokenType::RightParen)?;
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
                self.expect(TokenType::RightBracket)?;
                let expr = Expression::Array(ArrayExpression::new(array));
                return Ok(expr);
            },
            TokenType::LeftBrace => {
                // Dictionary
                self.next_token()?;
                let mut pairs = vec![];
                while !self.peek_is(TokenType::RightBrace)? {
                    let name = self.expect(TokenType::Identifier)?;
                    self.expect(TokenType::Colon)?;
                    let value = self.expression()?;
                    pairs.push((name, value));
                    if self.peek_is(TokenType::Comma)? {
                        self.next_token()?;
                    } else {
                        break;
                    }
                }
                let expr = DictionaryExpression::new(pairs).into();
                self.expect(TokenType::RightBrace)?;
                return Ok(expr);
            },
            _ => Err(ParserError::ExpectedLiteral(self.get_lex_snippet(), self.current_token.as_ref().map(|t| t.ty)))
        }?;
        loop {
            match self.peek()?.ty {
                TokenType::Dot => {
                    expr = self.accessor_expr(expr)?;
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
                    self.expect(TokenType::RightParen)?;
                    expr = FunctionCallExpression::new(expr, exprs).into();
                },
                TokenType::LeftBracket => {
                    self.next_token()?;
                    let indexer = self.expression()?;
                    self.expect(TokenType::RightBracket)?;
                    expr = IndexExpression::new(indexer, expr).into();
                },
                _ => break
            }
        }
        Ok(expr)
    }

    fn accessor_expr(&mut self, expr: Expression) -> Result<Expression, ParserError> {
        self.next_token()?;
        let field = self.next_token()?;
        Ok(AccessorExpression::new(expr, field).into())
    }

    fn expr_list(&mut self, delimiter: TokenType, separator: TokenType) -> Result<Vec<Expression>, ParserError> {
        let mut exprs = vec![];
        exprs.push(self.expression()?);
        while self.peek_is(separator)? && !self.peek_is(delimiter)? {
            self.next_token()?;
            exprs.push(self.expression()?);
        }
        Ok(exprs)
    }

    fn block_stat(&mut self) -> Result<Statement, ParserError> {
        let mut stats = vec![];
        self.expect(TokenType::LeftBrace)?;
        while !self.peek_is(TokenType::RightBrace)? {
            stats.push(self.statement()?);
        }
        self.expect(TokenType::RightBrace)?;
        Ok(BlockStatement::new(stats).into())
    }

    fn layout_decl(&mut self) -> Result<Declaration, ParserError> {
        self.expect(TokenType::KwLayout)?;
        let mut parameters = vec![];
        let name = if self.peek_is(TokenType::Identifier)? {
            let name = self.next_token()?;
            if self.peek_is(TokenType::LeftParen)? {
                self.next_token()?;
                loop {
                    parameters.push(self.expect(TokenType::Identifier)?);
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
        self.expect(TokenType::LeftBrace)?;
        let markup = self.markup_block()?;
        self.expect(TokenType::RightBrace)?;
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
                        self.expect(TokenType::RightBrace)?;
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
        let style_name = self.expect(TokenType::Identifier)?;
        self.expect(TokenType::Assign)?;
        let value = self.expression()?;
        self.expect(TokenType::Semicolon)?;
        Ok((style_name, value))
    }

    fn match_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(TokenType::KwMatch)?;
        let comparator = self.expression()?;
        self.expect(TokenType::LeftBrace)?;
        let mut cases = vec![];
        let mut default_case = None;
        while !self.peek_is(TokenType::RightBrace)? {
            if self.peek_is(TokenType::KwDefault)? {
                self.expect(TokenType::KwDefault)?;
                let token = if self.peek_is(TokenType::Identifier)? {
                    Some(self.next_token()?)
                } else {
                    None
                };
                self.expect(TokenType::Arrow)?;
                let markup = self.markup()?;
                if self.peek_is(TokenType::Comma)? {
                    self.next_token()?;
                }
                default_case = Some((token, markup));
                break;
            } else {
                let lhs = self.expression()?;
                self.expect(TokenType::Arrow)?;
                let rhs = if self.peek_is(TokenType::LeftBrace)? {
                    self.markup()?
                } else {
                    vec![self.markup_item()?]
                };
                if !self.peek_is(TokenType::RightBrace)? {
                    self.expect(TokenType::Comma)?;
                }
                cases.push((lhs, rhs));
            }
        }
        self.expect(TokenType::RightBrace)?;
        Ok(MatchConditionalRender::new(comparator, cases, default_case).into())
    }

    fn if_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(TokenType::KwIf)?;
        let condition = self.expression()?;
        let if_markup = self.markup_block()?;
        let else_markup = if self.peek_is(TokenType::KwElse)? {
        self.next_token()?;
            if self.peek_is(TokenType::KwIf)? {
                Some(self.if_markup()?)
            } else {
                Some(self.markup_block()?)
            }
        } else {
            None
        };
        Ok(IfConditionalRender::new(condition, if_markup, else_markup).into())
    }

    fn for_markup(&mut self) -> Result<Markup, ParserError> {
        self.expect(TokenType::KwFor)?;
        let name = self.expect(TokenType::Identifier)?;
        self.expect(TokenType::KwIn)?;
        let iterable = self.expression()?;
        let end_range = if self.peek_is(TokenType::Colon)? {
            self.next_token()?;
            Some(self.expression()?)
        } else {
            None
        };
        let block = Some(self.markup_block()?);
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
            self.expect(TokenType::RightParen)?;
            self.expect(TokenType::Semicolon)?;
            return Ok(LayoutRender::new(name, exprs).into())
        }
        let bindings = if self.peek_is(TokenType::Identifier)? {
            let mut bindings = vec![];
            while self.peek_is(TokenType::Identifier)? {
                let name = self.next_token()?;
                let mut is_lambda = false;
                let parameters = if self.peek_is(TokenType::LeftParen)? {
                    is_lambda = true;
                    self.expect(TokenType::LeftParen)?;
                    let mut parameters = vec![];
                    while !self.peek_is(TokenType::RightParen)? {
                        parameters.push(self.expect(TokenType::Identifier)?);
                        if self.peek_is(TokenType::Comma)? {
                            self.expect(TokenType::Comma)?;
                        } else {
                            self.expect(TokenType::RightParen)?;
                            break;
                        }
                    }
                    parameters
                } else {
                    vec![]
                };
                self.expect(TokenType::Assign)?;
                if self.peek_is(TokenType::LeftBrace)? {
                    is_lambda = true;
                }
                let binding = if is_lambda {
                    self.expect(TokenType::LeftBrace)?;
                    let exprs = self.expr_list(TokenType::RightBrace, TokenType::Semicolon)?;
                    self.expect(TokenType::RightBrace)?;
                    LambdaFunctionBinding::new(parameters, exprs).into()
                } else {
                    let tokens = if self.peek_is(TokenType::LeftBracket)? {
                        let mut tokens = vec![];
                        self.next_token()?;
                        while !self.peek_is(TokenType::RightBracket)? {
                            tokens.push(self.expect(TokenType::Identifier)?);
                            if self.peek_is(TokenType::Comma)? {
                                self.next_token()?;
                            } else {
                                break;
                            }
                        }
                        self.expect(TokenType::RightBracket)?;
                        tokens
                    } else {
                        vec![self.expect(TokenType::Identifier)?]
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
            self.expect(TokenType::Semicolon)?;
            None
        } else if self.peek_is(TokenType::LeftBrace)? {
            Some(self.markup_block()?)
        } else {
            None
        };
        Ok(ComponentRender::new(name, bindings, string_literal, children).into())
    }

    fn markup_block(&mut self) -> Result<Markup, ParserError> {
        self.expect(TokenType::LeftBrace)?;
        let mut markup = self.markup()?;
        self.expect(TokenType::RightBrace)?;
        Ok(MarkupBlock::new(markup).into())
    }
}
