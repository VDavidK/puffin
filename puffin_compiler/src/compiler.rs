use crate::scope::Scope;
use puffin_ast::Ast;
use puffin_ast::declaration::Declaration;
use puffin_ast::expression::Expression;
use puffin_ast::markup::{ComponentParameter, ComponentRender, Markup};
use puffin_ast::statement::Statement;
use puffin_ast::token::{Token, TokenType};
use puffin_runtime::chunk::{ConstantOffset, LocalOffset};
use puffin_runtime::op::OpCode;
use puffin_runtime::value::{FloatType, Function, IntType};
use puffin_runtime::{Chunk, value::Value};
use std::collections::HashMap;
use std::rc::Rc;

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
    IncorrectFunctionCallArity,

    #[error("Cannot pop top scope")]
    TopScopePopped,
}

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    constant_table: HashMap<Value, ConstantOffset>,
    scope: Scope<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            constant_table: HashMap::new(),
            scope: Scope::new(),
        }
    }

    pub fn compile(&mut self, ast: &'a Ast) -> Result<(), CompileError> {
        let name = self.add_to_constants(ast.component_name.to_owned())?;
        self.chunk.push_op(OpCode::NewClass);
        self.chunk.push_constant_offset(name);

        for decl in &ast.declarations {
            self.compile_declaration(decl)?;
        }

        self.chunk.push_op(OpCode::SetGlobal);
        self.chunk.push_constant_offset(name);

        Ok(())
    }

    fn compile_declaration(&mut self, declaration: &'a Declaration) -> Result<(), CompileError> {
        match declaration {
            // Declaration::Component(_) => {}
            Declaration::Var(var) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);

                let name = self.chunk.new_constant(var.name.lexeme.clone());
                self.compile_expression(&var.value)?;

                self.chunk.push_op(OpCode::SetField);
                self.chunk.push_constant_offset(name);
            }
            Declaration::Layout(layout) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);

                let name = layout.name
                    .as_ref()
                    .map(|name| name.lexeme.as_str())
                    .unwrap_or("<main>")
                    .to_owned();

                let mut chunk = Chunk::new(&name);
                let mut layout_compiler = Compiler::new(&mut chunk);

                for arg in &layout.parameters {
                    layout_compiler.scope.define_local(&arg.lexeme);
                }

                layout_compiler.compile_markup(&layout.markup)?;
                layout_compiler.chunk.push_op(OpCode::Return);

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: name.to_owned(),
                    arity: layout.parameters.len() + 1, // One more for self
                };

                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.chunk.push_op(OpCode::SetField);
                let name = self.add_to_constants(name)?;
                self.chunk.push_constant_offset(name);
            }
            // Declaration::Signal(_) => {}
            Declaration::Method(method) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);

                let mut chunk = Chunk::new(method.name.lexeme.clone());
                let mut method_compiler = Compiler::new(&mut chunk);

                for arg in &method.parameters {
                    method_compiler.scope.define_local(&arg.lexeme);
                }

                method_compiler.compile_statement(&method.block)?;
                method_compiler.ensure_return()?;

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: method.name.lexeme.clone(),
                    arity: method.parameters.len(),
                };
                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.chunk.push_op(OpCode::SetField);
                let name = self.token_to_constant(&method.name)?;
                self.chunk.push_constant_offset(name);
            }
            Declaration::Constructor(constructor) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);

                let mut chunk = Chunk::new("constructor");
                let mut method_compiler = Compiler::new(&mut chunk);

                for arg in &constructor.parameters {
                    method_compiler.scope.define_local(&arg.lexeme);
                }

                method_compiler.compile_statement(&constructor.block)?;
                method_compiler.ensure_return()?;

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: "constructor".to_owned(),
                    arity: constructor.parameters.len(),
                };
                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.chunk.push_op(OpCode::SetConstructor);
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
                self.push_scope();
                for stmt in &block.statements {
                    self.compile_statement(stmt)?;
                }
                self.pop_scope()?;
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
            Statement::For(stmt) => {
                if let Some(end) = &stmt.end_range {
                    //   <start> #start_local
                    //   <end>   #end_local
                    //   getl #start_local
                    //   getl #end_local
                    //   lt
                    //   jumpi :end
                    // loop:
                    //   <body>
                    //   getl #start_local
                    //   const 1
                    //   add
                    //   setl #start_local
                    //   getl #start_local
                    //   getl #end_local
                    //   lt
                    //   jumpi :loop
                    // end:

                    self.push_scope();
                    self.compile_expression(&stmt.iterable)?;
                    let start_local = self.scope.define_local(&stmt.var_name.lexeme);
                    self.compile_expression(end)?;
                    let end_local = self.scope.define_unnamed_local();

                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(end_local);
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(start_local);
                    self.chunk.push_op(OpCode::Lt);
                    let end_jump = self.chunk.push_jump(OpCode::JumpIf);

                    let one = self.add_to_constants(1)?;

                    let loop_addr = self.chunk.addr();
                    self.compile_statement(&stmt.block)?;
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(start_local);
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(one);
                    self.chunk.push_op(OpCode::Add);
                    self.chunk.push_op(OpCode::SetLocal);
                    self.chunk.push_local_offset(start_local);
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(start_local);
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(end_local);
                    self.chunk.push_op(OpCode::Lt);
                    self.chunk.push_jump_im(OpCode::JumpIf, loop_addr);

                    self.chunk.patch_jump(end_jump, self.chunk.addr());
                    self.pop_scope()?;
                } else {
                    todo!("Generic for loops not supported yet");
                }
            }

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
            Statement::Return(ret) => {
                if let Some(expr) = &ret.expression {
                    self.compile_expression(expr)?;
                } else {
                    let null = self.add_to_constants(Value::Null)?;
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(null);
                }
                self.chunk.push_op(OpCode::Return);
            }
            // Statement::Match(_) => {}
            Statement::VariableDeclaration(var) => {
                self.compile_expression(&var.value)?;
                self.scope.define_local(&var.name.lexeme);
            }
            _ => (),
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: &'a Expression) -> Result<(), CompileError> {
        match expression {
            Expression::Literal(literal) => {
                if literal.token.ty == TokenType::Identifier {
                    match self.scope.lookup_local(literal.token.lexeme.as_str()) {
                        None => {
                            let global = self.token_to_constant(&literal.token)?;
                            self.chunk.push_op(OpCode::GetGlobal);
                            self.chunk.push_constant_offset(global);
                        }
                        Some(addr) => {
                            self.chunk.push_op(OpCode::GetLocal);
                            self.chunk.push_local_offset(addr);
                        }
                    }
                } else {
                    let constant = self.token_to_constant(&literal.token)?;
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(constant);
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
                self.chunk.push_u8(call.arguments.len() as u8);
            }
            Expression::Accessor(accessor) => {
                let name = self.token_to_constant(&accessor.field)?;
                self.compile_expression(&accessor.expression)?;
                self.chunk.push_op(OpCode::GetField);
                self.chunk.push_constant_offset(name);
            }
            Expression::Array(arr) => {
                self.chunk.push_op(OpCode::NewList);
                let list = self.scope.define_unnamed_local();

                for elem in &arr.entries {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(list);
                    self.compile_expression(&elem)?;
                    self.chunk.push_op(OpCode::PushList);
                }
            }
            Expression::Dictionary(dict) => {
                self.chunk.push_op(OpCode::NewInstance);
                let obj = self.scope.define_unnamed_local();

                for (key, value) in &dict.entries {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(obj);
                    self.compile_expression(value)?;
                    self.chunk.push_op(OpCode::SetField);
                    let name = self.token_to_constant(key)?;
                    self.chunk.push_constant_offset(name);
                }
            }
            // Expression::Match(_) => {}
            _ => (),
        }

        Ok(())
    }

    fn compile_markup(&mut self, markup: &'a Markup) -> Result<(), CompileError> {
        match markup {
            Markup::Block(block) => {
                self.chunk.push_op(OpCode::NewList);
                let children = self.scope.define_unnamed_local();

                for elem in &block.markup {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(children);
                    self.scope.define_unnamed_local();

                    self.compile_markup(elem)?;

                    self.scope.remove_top_local();
                    self.chunk.push_op(OpCode::PushList);
                }
            }
            Markup::Component(component) => {
                let mut arg_count = 0;

                let arg_count = match &component.parameter {
                    None => 0,
                    Some(ComponentParameter::Expression(expr)) => {
                        self.compile_expression(expr)?;
                        1
                    }
                    Some(ComponentParameter::Children(inner)) => {
                        self.compile_markup(inner)?;
                        1
                    }
                };

                let global = self.token_to_constant(&component.name)?;
                self.chunk.push_op(OpCode::GetGlobal);
                self.chunk.push_constant_offset(global);
                self.chunk.push_op(OpCode::Call);
                self.chunk.push_u8(arg_count);
            }
            // Markup::Layout(_) => {}
            // Markup::Match(_) => {}
            Markup::If(markup) => {
                self.compile_expression(&markup.condition)?;
                self.chunk.push_op(OpCode::Not);
                let jmp = self.chunk.push_jump(OpCode::JumpIf);

                self.compile_markup(&markup.if_markup)?;

                // TODO: ???
                // let end_addr = if let Some(body) = &markup.else_markup {
                //     let if_to_end_jump = self.chunk.push_jump(OpCode::Jump);
                //     let else_addr = self.chunk.addr();
                //     self.compile_statement(else_stmt)?;
                //     self.chunk.patch_jump(if_to_end_jump, self.chunk.addr());
                //     else_addr
                // } else {
                //     self.chunk.addr()
                // };
                let end_addr = self.chunk.addr();

                self.chunk.patch_jump(jmp, end_addr);

                // TODO: Should not exist
                // if_markup.elseif_markup
            }
            // Markup::Iterative(_) => {}
            // Markup::Style(_) => {}
            _ => (),
        }

        Ok(())
    }

    fn compile_component(&mut self, component: &ComponentRender) -> Result<(), CompileError> {
        Ok(())
    }

    fn ensure_return(&mut self) -> Result<(), CompileError> {
        if !self.chunk.last_op_is(OpCode::Return) {
            self.chunk.push_op(OpCode::Constant);
            let null = self.add_to_constants(Value::Null)?;
            self.chunk.push_constant_offset(null);
            self.chunk.push_op(OpCode::Return);
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
            }

            Expression::Literal(literal) => {
                match self.scope.lookup_local(literal.token.lexeme.as_str()) {
                    None => {
                        let global = self.token_to_constant(&literal.token)?;
                        Ok(VariableTarget::Global(global))
                    }
                    Some(addr) => Ok(VariableTarget::Local(addr)),
                }
            }

            _ => Err(CompileError::InvalidTarget),
        }
    }

    fn token_to_value(&self, token: &Token) -> Result<Value, CompileError> {
        match token.ty {
            TokenType::KwTrue => Ok(Value::Bool(true)),
            TokenType::KwFalse => Ok(Value::Bool(false)),
            TokenType::KwNull => Ok(Value::Null),
            TokenType::Integer => {
                let val = token
                    .lexeme
                    .parse::<IntType>()
                    .map_err(|_| CompileError::InvalidLiteral(token.lexeme.clone()))?;

                Ok(Value::Int(val))
            }
            TokenType::Float => {
                let val = token
                    .lexeme
                    .parse::<FloatType>()
                    .map_err(|_| CompileError::InvalidLiteral(token.lexeme.clone()))?;

                Ok(Value::Float(val))
            }
            TokenType::String => Ok(Value::String(
                token.lexeme[1..token.lexeme.len() - 1].to_owned(),
            )),
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

    fn add_to_constants(
        &mut self,
        constant: impl Into<Value>,
    ) -> Result<ConstantOffset, CompileError> {
        let constant = constant.into();

        if let Some(addr) = self.constant_table.get(&constant) {
            Ok(*addr)
        } else {
            let offset = self.chunk.new_constant(constant.clone());
            self.constant_table.insert(constant, offset);
            Ok(offset)
        }
    }

    fn push_scope(&mut self) {
        let old_scope = std::mem::replace(&mut self.scope, Scope::new());
        self.scope.set_parent(Box::new(old_scope));
    }

    fn pop_scope(&mut self) -> Result<(), CompileError> {
        match self.scope.remove_parent() {
            Some(parent) => {
                for _ in 0..self.scope.local_count() {
                    self.chunk.push_op(OpCode::Pop);
                }

                self.scope = *parent;
                Ok(())
            }
            None => Err(CompileError::TopScopePopped),
        }
    }
}

enum VariableTarget {
    Local(LocalOffset),
    Global(ConstantOffset),
    Object(ConstantOffset),
}
