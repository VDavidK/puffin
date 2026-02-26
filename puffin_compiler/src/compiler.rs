use puffin_ast::Ast;
use puffin_ast::declaration::Declaration;
use puffin_ast::expression::Expression;
use puffin_ast::statement::Statement;
use puffin_ast::token::{Token, TokenType};
use puffin_runtime::{Chunk, Value};
use puffin_runtime::value::{FloatType, IntType};

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("Invalid integer literal '{0}'")]
    InvalidIntegerLiteral(String),

    #[error("Invalid float literal '{0}'")]
    InvalidFloatLiteral(String),
}

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk
        }
    }

    pub fn compile(&mut self, ast: &Ast) {
        for decl in &ast.declarations {
            self.compile_declaration(decl);
        }
    }

    fn compile_declaration(&mut self, declaration: &Declaration) {
        match declaration {
            // Declaration::Component(_) => {}
            // Declaration::Var(_) => {}
            // Declaration::Layout(_) => {}
            // Declaration::Signal(_) => {}
            Declaration::Method(method) => {
                self.compile_statement(&method.block);
            }
            // Declaration::Require(_) => {}
            // Declaration::Use(_) => {}
            // Declaration::Export(_) => {}
            // Declaration::Enum(_) => {}
            _ => (),
        }
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Block(block) => {
                for stmt in &block.statements {
                    self.compile_statement(stmt);
                }
            }
            // Statement::Assign(_) => {}
            // Statement::Var(_) => {}
            // Statement::Break(_) => {}
            // Statement::Continue(_) => {}
            // Statement::For(_) => {}
            // Statement::If(_) => {}
            Statement::Expression(expr_stmt) => {
                self.compile_expression(&expr_stmt.expression);
            }
            // Statement::Return(_) => {}
            // Statement::Match(_) => {}
            // Statement::VariableDeclaration(_) => {}
            _ => (),
        }
    }

    fn compile_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Literal(literal) => {
            }
            // Expression::Binary(_) => {}
            // Expression::Unary(_) => {}
            // Expression::FunctionCall(_) => {}
            // Expression::Accessor(_) => {}
            // Expression::Array(_) => {}
            // Expression::Dictionary(_) => {}
            // Expression::Match(_) => {}
            _ => (),
        }
    }

    fn token_to_value(token: &Token) -> Result<Value, CompileError> {
        match token.ty {
            TokenType::KwTrue => Ok(Value::Bool(true)),
            TokenType::KwFalse => Ok(Value::Bool(false)),
            // TokenType::KwNull => Ok(Value::Null),
            TokenType::Integer => {
                let val = token.lexeme
                    .parse::<IntType>()
                    .map_err(|_| CompileError::InvalidIntegerLiteral(token.lexeme.clone()))?;

                Ok(Value::Int(val))
            },
            TokenType::Float => {
                let val = token.lexeme
                    .parse::<FloatType>()
                    .map_err(|_| CompileError::InvalidFloatLiteral(token.lexeme.clone()))?;

                Ok(Value::Float(val))
            }
            // TokenType::String => {
            // }
            // TokenType::Identifier => {}
            _ => todo!(),
        }
    }
}
