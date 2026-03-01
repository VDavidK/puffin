use std::collections::HashMap;
use puffin_ast::Ast;
use puffin_ast::declaration::Declaration;
use puffin_ast::expression::Expression;
use puffin_ast::statement::Statement;
use puffin_ast::token::{Token, TokenType};
use puffin_runtime::{Chunk, Value};
use puffin_runtime::chunk::{ConstantOffset, LocalOffset};
use puffin_runtime::op::OpCode;
use puffin_runtime::value::{FloatType, IntType};

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("Invalid literal '{0}'")]
    InvalidLiteral(String),

    #[error("Invalid binary operator '{0}'")]
    InvalidBinaryOperator(TokenType),

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Expression is not a valid target")]
    InvalidTarget,

    #[error("Incorrect number of arguments")]
    IncorrectFunctionCallArity
}

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    constant_table: HashMap<Value, ConstantOffset>,
    local_table: HashMap<&'a str, LocalOffset>,
    local_count: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            constant_table: HashMap::new(),
            local_table: HashMap::new(),
            local_count: 0,
        }
    }

    pub fn compile(&mut self, ast: &'a Ast) -> Result<(), CompileError> {
        for decl in &ast.declarations {
            self.compile_declaration(decl)?;
        }

        Ok(())
    }

    fn compile_declaration(&mut self, declaration: &'a Declaration) -> Result<(), CompileError> {
        match declaration {
            // Declaration::Component(_) => {}
            Declaration::Var(var) => {
                let name = self.chunk.new_constant(var.name.lexeme.clone());
                self.compile_expression(&var.value)?;
                self.chunk.push_op(OpCode::SetGlobal);
                self.chunk.push_constant_offset(name);
            }
            // Declaration::Layout(_) => {}
            // Declaration::Signal(_) => {}
            Declaration::Method(method) => {
                self.compile_statement(&method.block)?;
            }
            // Declaration::Require(_) => {}
            // Declaration::Use(_) => {}
            // Declaration::Export(_) => {}
            // Declaration::Enum(_) => {}
            _ => (),
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: &'a Statement) -> Result<(), CompileError> {
        //
        match statement {
            Statement::Block(block) => {
                for stmt in &block.statements {
                    self.compile_statement(stmt)?;
                }
            }
            Statement::Assign(assign) => {
                self.compile_expression(&assign.rhs)?;
                match self.fetch_target(&assign.lhs)? {
                    VariableTarget::Local(local) => {
                        self.chunk.push_op(OpCode::SetLocal);
                        self.chunk.push_local_offset(local);
                    }
                    VariableTarget::Global(global) => {
                        self.chunk.push_op(OpCode::SetGlobal);
                        self.chunk.push_constant_offset(global);
                    }
                    VariableTarget::Object(obj) => {
                        self.chunk.push_op(OpCode::SetField);
                        self.chunk.push_constant_offset(obj);
                    }
                }
            }
            // Statement::Break(_) => {}
            // Statement::Continue(_) => {}
            // Statement::For(_) => {}
            Statement::If(stmt) => {
                self.compile_expression(&stmt.condition)?;
                self.chunk.push_op(OpCode::Not);
                let jmp = self.chunk.push_jump(OpCode::JumpIf);
                self.compile_statement(&stmt.if_block)?;

                let end_addr = if let Some(else_stmt) = &stmt.else_stat {
                    let if_to_end_jump = self.chunk.push_jump(OpCode::Jump);
                    let else_addr = self.chunk.addr();
                    self.compile_statement(else_stmt)?;
                    self.chunk.patch_jump(if_to_end_jump, self.chunk.addr());
                    else_addr
                } else {
                    self.chunk.addr()
                };

                self.chunk.patch_jump(jmp, end_addr);
            }
            Statement::Expression(expr_stmt) => {
                self.compile_expression(&expr_stmt.expression)?;
                self.chunk.push_op(OpCode::Pop);
            }
            // Statement::Return(_) => {}
            // Statement::Match(_) => {}
            Statement::VariableDeclaration(var) => {
                self.compile_expression(&var.value)?;
                self.local_table.insert(&var.name.lexeme, self.local_count as LocalOffset);

                self.local_count += 1;
            }
            _ => (),
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: &'a Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Literal(literal) => {
                if literal.token.ty == TokenType::Identifier {
                    match self.local_table.get(literal.token.lexeme.as_str()) {
                        None => {
                            let global = self.token_to_constant(&literal.token)?;
                            self.chunk.push_op(OpCode::GetGlobal);
                            self.chunk.push_constant_offset(global);
                        },
                        Some(addr) => {
                            self.chunk.push_op(OpCode::GetLocal);
                            self.chunk.push_local_offset(*addr);
                        }
                    }
                } else {
                    let constant = self.token_to_value(&literal.token)?;
                    self.chunk.push_constant(constant);
                }
            }
            Expression::Binary(binary) => {
                self.compile_expression(&binary.lhs)?;
                self.compile_expression(&binary.rhs)?;
                self.chunk.push_op(Self::get_binary_operator(binary.op.ty)?);
            }
            Expression::Unary(unary) => {
                self.compile_expression(&unary.rhs)?;

                if unary.op.ty != TokenType::Plus {
                    self.chunk.push_op(Self::get_unary_operator(unary.op.ty)?);
                }
            }
            Expression::FunctionCall(call) => {
                for arg in &call.arguments {
                    self.compile_expression(arg)?;
                }
                self.compile_expression(&call.callee)?;
                self.chunk.push_op(OpCode::Call);
            }
            Expression::Accessor(accessor) => {
                let name = self.token_to_constant(&accessor.field)?;
                self.compile_expression(&accessor.expression)?;
                self.chunk.push_op(OpCode::GetField);
                self.chunk.push_constant_offset(name);
            }
            // Expression::Array(_) => {}
            // Expression::Dictionary(_) => {}
            // Expression::Match(_) => {}
            _ => (),
        }

        Ok(())
    }

    fn fetch_target(&mut self, expr: &'a Expression) -> Result<VariableTarget, CompileError> {
        match expr {
            Expression::Accessor(accessor) => {
                match self.fetch_target(expr)? {
                    VariableTarget::Local(local) => {
                        self.chunk.push_op(OpCode::GetLocal);
                        self.chunk.push_local_offset(local);
                    }
                    VariableTarget::Global(global) => {
                        self.chunk.push_op(OpCode::GetGlobal);
                        self.chunk.push_constant_offset(global);
                    }
                    VariableTarget::Object(obj) => {
                        self.chunk.push_op(OpCode::GetField);
                        self.chunk.push_constant_offset(obj);
                    }
                };
                let name = self.token_to_constant(&accessor.field)?;
                Ok(VariableTarget::Object(name))
            },

            Expression::Literal(literal) => match self.local_table.get(literal.token.lexeme.as_str()) {
                None => {
                    let global = self.token_to_constant(&literal.token)?;
                    Ok(VariableTarget::Global(global))
                },
                Some(addr) => {
                    Ok(VariableTarget::Local(*addr))
                }
            },

            _ => Err(CompileError::InvalidTarget),
        }
    }


    fn token_to_value(&self, token: &Token) -> Result<Value, CompileError> {
        match token.ty {
            TokenType::KwTrue => Ok(Value::Bool(true)),
            TokenType::KwFalse => Ok(Value::Bool(false)),
            TokenType::KwNull => Ok(Value::Null),
            TokenType::Integer => {
                let val = token.lexeme
                    .parse::<IntType>()
                    .map_err(|_| CompileError::InvalidLiteral(token.lexeme.clone()))?;

                Ok(Value::Int(val))
            },
            TokenType::Float => {
                let val = token.lexeme
                    .parse::<FloatType>()
                    .map_err(|_| CompileError::InvalidLiteral(token.lexeme.clone()))?;

                Ok(Value::Float(val))
            },
            TokenType::String => Ok(Value::String(token.lexeme[1..token.lexeme.len() - 1].to_owned())),
            TokenType::Identifier => Ok(Value::String(token.lexeme.to_owned())),

            _ => Err(CompileError::InvalidLiteral(token.lexeme.clone())),
        }
    }

    fn get_binary_operator(ty: TokenType) -> Result<OpCode, CompileError> {
        match ty {
            TokenType::KwAnd => todo!(),
            TokenType::KwOr => todo!(),
            TokenType::Plus => Ok(OpCode::Add),
            TokenType::Minus => Ok(OpCode::Sub),
            TokenType::Star => Ok(OpCode::Mul),
            TokenType::Slash => Ok(OpCode::Div),
            TokenType::IsEqualTo => Ok(OpCode::Eq),
            TokenType::IsNotEqualTo => Ok(OpCode::Neq),
            TokenType::GreaterThan => Ok(OpCode::Gt),
            TokenType::LessThan => Ok(OpCode::Lt),
            TokenType::GreaterOrEqual => Ok(OpCode::Ge),
            TokenType::LessOrEqual => Ok(OpCode::Le),

            _ => Err(CompileError::InvalidBinaryOperator(ty)),
        }
    }

    fn get_unary_operator(ty: TokenType) -> Result<OpCode, CompileError> {
        match ty {
            TokenType::KwNot => Ok(OpCode::Not),
            TokenType::Minus => Ok(OpCode::Neg),
            _ => Err(CompileError::InvalidBinaryOperator(ty)),
        }
    }

    fn token_to_constant(&mut self, token: &Token) -> Result<ConstantOffset, CompileError> {
        let constant = self.token_to_value(token)?;
        self.add_to_constants(constant)
    }

    fn add_to_constants(&mut self, constant: impl Into<Value>) -> Result<ConstantOffset, CompileError> {
        let constant = constant.into();

        if let Some(addr) = self.constant_table.get(&constant) {
            Ok(*addr)
        } else {
            Ok(self.chunk.new_constant(constant))
        }
    }
}

enum VariableTarget {
    Local(LocalOffset),
    Global(ConstantOffset),
    Object(ConstantOffset),
}
