use std::cell::RefCell;
use crate::scope::Scope;
use puffin_ast::Ast;
use puffin_ast::declaration::Declaration;
use puffin_ast::expression::{BinaryExpression, Expression};
use puffin_ast::markup::{ComponentParameter, Markup};
use puffin_ast::statement::Statement;
use puffin_ast::token::{Token, TokenType};
use puffin_runtime::chunk::{ConstantOffset, LocalOffset};
use puffin_runtime::op::OpCode;
use puffin_runtime::value::{FloatType, Function, IntType};
use puffin_runtime::{Chunk, value::Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("Invalid literal '{0}'")]
    InvalidLiteral(String),

    #[error("Invalid binary operator '{0}'")]
    InvalidBinaryOperator(TokenType),

    #[error("Invalid shorthand operator '{0}'")]
    InvalidShorthandOperator(TokenType),

    #[error("Variable '{0}' not found")]
    VariableNotFound(String),

    #[error("Expression is not a valid target")]
    InvalidTarget,

    #[error("Incorrect number of arguments")]
    IncorrectFunctionCallArity,

    #[error("Cannot pop top scope")]
    TopScopePopped,

    #[error("Invalid 'use' path")]
    InvalidUsePath,
}

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    constant_table: HashMap<Value, ConstantOffset>,
    markup_declarations: HashMap<String, LocalOffset>,
    dependencies: Vec<PathBuf>,
    scope: Scope,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            constant_table: HashMap::new(),
            markup_declarations: HashMap::new(),
            dependencies: vec![],
            scope: Scope::new(),
        }
    }

    pub fn compile(&mut self, ast: &'a Ast) -> Result<(), CompileError> {
        let name = self.add_to_constants(ast.component_name.to_owned())?;
        self.chunk.push_op(OpCode::NewClass);
        self.chunk.push_constant_offset(name);
        self.scope.define_unnamed_local();

        for decl in &ast.declarations {
            self.compile_declaration(decl)?;
        }

        self.chunk.push_op(OpCode::SetGlobal);
        self.chunk.push_constant_offset(name);
        self.scope.remove_top_local();
        Ok(())
    }

    pub fn get_dependencies(self) -> Vec<PathBuf> {
        self.dependencies.to_owned()
    }

    fn compile_declaration(&mut self, declaration: &'a Declaration) -> Result<(), CompileError> {
        match declaration {
            // Declaration::Component(_) => {}
            Declaration::Var(var) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);
                self.scope.define_unnamed_local();

                let name = self.chunk.new_constant(var.name.lexeme.clone());
                self.compile_expression(&var.value)?;
                self.chunk.push_op(OpCode::MakeReactive);

                self.chunk.push_op(OpCode::SetField);
                self.chunk.push_constant_offset(name);
                self.scope.remove_top_n_locals(2);
            }
            Declaration::Layout(layout) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);
                self.scope.define_unnamed_local();

                let name = layout.name
                    .as_ref()
                    .map(|name| name.lexeme.as_str())
                    .unwrap_or("<construct>")
                    .to_owned();

                let mut chunk = Chunk::new(&name);
                let mut layout_compiler = Compiler::new(&mut chunk);

                for arg in &layout.parameters {
                    layout_compiler.scope.define_local(&arg.lexeme);
                }

                layout_compiler.scope.define_local("self");

                layout_compiler.compile_markup(&layout.markup)?;
                layout_compiler.chunk.push_op(OpCode::Return);

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: name.to_owned(),
                    arity: layout.parameters.len(),
                    bound_value: None,
                };

                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.scope.define_unnamed_local();
                self.chunk.push_op(OpCode::SetClassMethod);
                let name = self.add_to_constants(name)?;
                self.chunk.push_constant_offset(name);
                self.scope.remove_top_n_locals(2);
            }
            // Declaration::Signal(_) => {}
            Declaration::Method(method) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);
                self.scope.define_unnamed_local();

                let mut chunk = Chunk::new(method.name.lexeme.clone());
                let mut method_compiler = Compiler::new(&mut chunk);

                for arg in &method.parameters {
                    method_compiler.scope.define_local(&arg.lexeme);
                }

                method_compiler.scope.define_local("self");

                method_compiler.compile_statement(&method.block)?;
                method_compiler.ensure_return()?;

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: method.name.lexeme.clone(),
                    arity: method.parameters.len(),
                    bound_value: None,
                };
                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.scope.define_unnamed_local();
                if let Some(decorator) = &method.decorator {
                    self.chunk.push_op(OpCode::SetHandler);
                    let name = self.token_to_constant(&decorator.name)?;
                    self.chunk.push_constant_offset(name);
                } else {
                    self.chunk.push_op(OpCode::SetClassMethod);
                    let name = self.token_to_constant(&method.name)?;
                    self.chunk.push_constant_offset(name);
                }
                self.scope.remove_top_n_locals(2);
            }
            Declaration::Constructor(constructor) => {
                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_constant_offset(0);
                self.scope.define_unnamed_local();

                let mut chunk = Chunk::new("constructor");
                let mut method_compiler = Compiler::new(&mut chunk);

                for arg in &constructor.parameters {
                    method_compiler.scope.define_local(&arg.lexeme);
                }

                method_compiler.scope.define_local("self");

                method_compiler.compile_statement(&constructor.block)?;
                method_compiler.ensure_return()?;

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: "constructor".to_owned(),
                    arity: constructor.parameters.len(),
                    bound_value: None,
                };
                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.scope.define_unnamed_local();
                self.chunk.push_op(OpCode::SetConstructor);
                self.scope.remove_top_n_locals(2);
            }
            // Declaration::Require(_) => {}
            Declaration::Use(use_decl) => {
                let path = Self::expr_to_filepath(&use_decl.name)?;
                self.dependencies.push(path.into());
            }
            // Declaration::Export(_) => {}
            // Declaration::Enum(_) => {}
            _ => (),
        }

        Ok(())
    }

    fn expr_to_filepath(expr: &Expression) -> Result<String, CompileError> {
        let mut names = vec![];
        Self::extract_file_path_from_accessor_expr(expr, &mut names)?;
        let mut path = names.join("/");
        path.push_str(".puff");
        Ok(path)
    }

    fn extract_file_path_from_accessor_expr(expr: &Expression, v: &mut Vec<String>) -> Result<(), CompileError> {
        match expr {
            Expression::Literal(ex) => {
                v.push(ex.token.lexeme.to_owned());
            }
            Expression::Accessor(acc) => {
                Self::extract_file_path_from_accessor_expr(&acc.expression, v)?;
                v.push(acc.field.lexeme.to_owned());
            }
            _ => return Err(CompileError::InvalidUsePath)?,
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
                let target = self.fetch_target(&assign.lhs)?;
                self.compile_expression(&assign.rhs)?;
                match target {
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
                        self.scope.remove_top_local();
                    }
                }
                self.scope.remove_top_local();
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
                    let start_local = self.scope.replace_local(&stmt.var_name.lexeme);
                    self.compile_expression(end)?;
                    let end_local = self.scope.get_top_local();

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
                self.scope.remove_top_local();

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
                self.scope.remove_top_local();
            }
            Statement::Return(ret) => {
                if let Some(expr) = &ret.expression {
                    self.compile_expression(expr)?;
                } else {
                    let null = self.add_to_constants(Value::Null)?;
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(null);
                    self.scope.define_unnamed_local();
                }
                self.chunk.push_op(OpCode::Return);
                self.scope.remove_top_local();
            }
            // Statement::Match(_) => {}
            Statement::VariableDeclaration(var) => {
                self.compile_expression(&var.value)?;
                self.scope.replace_local(&var.name.lexeme);
            }
            Statement::Break(_) => todo!(),
            Statement::Continue(_) => todo!(),
            Statement::Match(_) => todo!(),
            Statement::Increment(increment) => {
                let target = self.fetch_target(&increment.target)?;
                self.compile_expression(&increment.target)?;
                let offset = self.add_to_constants(1)?;
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(offset);
                self.chunk.push_op(OpCode::Add);
                match target {
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
                        self.scope.remove_top_local();
                    }
                }
                self.scope.remove_top_local();
            },
            Statement::Decrement(decrement) => {
                let target = self.fetch_target(&decrement.target)?;
                self.compile_expression(&decrement.target)?;
                let offset = self.add_to_constants(1)?;
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(offset);
                self.chunk.push_op(OpCode::Sub);
                match target {
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
                        self.scope.remove_top_local();
                    }
                }
                self.scope.remove_top_local();
            },
            Statement::OpAssign(op_assign) => {
                let target = self.fetch_target(&op_assign.lhs)?;
                self.compile_expression(&op_assign.lhs)?;
                self.compile_expression(&op_assign.rhs)?;
                let op = Self::get_shorthand_operator(op_assign.op.ty)?;
                self.chunk.push_op(op);
                match target {
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
                        self.scope.remove_top_local();
                    }
                }
                self.scope.remove_top_n_locals(2);
            },
            Statement::Throw(_) => todo!(),
            Statement::Catch(_) => todo!(),
            Statement::Raise(_) => todo!(),
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
                            self.scope.define_unnamed_local();
                        }
                        Some(addr) => {
                            self.chunk.push_op(OpCode::GetLocal);
                            self.chunk.push_local_offset(addr);
                            self.scope.define_unnamed_local();
                        }
                    }
                } else {
                    let constant = self.token_to_constant(&literal.token)?;
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(constant);
                    self.scope.define_unnamed_local();
                }
            }
            Expression::Binary(BinaryExpression{ lhs, rhs, op: Token { ty: t @ (TokenType::KwAnd | TokenType::KwOr), .. }}) => {
                self.compile_expression(lhs)?;
                let loc = self.scope.get_top_local();

                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_local_offset(loc);

                if let TokenType::KwAnd = t {
                    self.chunk.push_op(OpCode::Not);
                }
                let jmp = self.chunk.push_jump(OpCode::JumpIf);
                self.chunk.push_op(OpCode::Pop);
                self.scope.remove_top_local();

                self.compile_expression(rhs)?;

                let chunk_addr = self.chunk.addr();
                self.chunk.patch_jump(jmp, chunk_addr);
            }
            Expression::Binary(binary) => {
                self.compile_expression(&binary.lhs)?;
                self.compile_expression(&binary.rhs)?;
                self.chunk.push_op(Self::get_binary_operator(binary.op.ty)?);
                self.scope.remove_top_local();
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
                for _ in &call.arguments {
                    self.scope.remove_top_local();
                }
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
                    self.compile_expression(elem)?;
                    self.chunk.push_op(OpCode::PushList);
                    self.scope.remove_top_local();
                }
            }
            Expression::Dictionary(dict) => {
                self.chunk.push_op(OpCode::NewDictionary);
                let obj = self.scope.define_unnamed_local();

                for (key, value) in &dict.entries {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(obj);
                    self.compile_expression(value)?;
                    self.chunk.push_op(OpCode::SetField);
                    let name = self.token_to_constant(key)?;
                    self.chunk.push_constant_offset(name);
                    self.scope.remove_top_local();
                }
            },
            Expression::Index(e) => {
                self.compile_expression(&e.expression)?;
                self.compile_expression(&e.index)?;
                self.chunk.push_op(OpCode::GetIndex);
                self.scope.remove_top_local();
            },
            Expression::Match(_) => todo!(),
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
                    self.chunk.push_op(OpCode::PushList);
                    self.scope.remove_top_n_locals(2);
                }

                self.chunk.push_op(OpCode::NewNodeBlock);
            }
            Markup::Component(component) => {
                match &component.parameter {
                    None => {
                        self.chunk.push_op(OpCode::Constant);
                        let null = self.add_to_constants(Value::Null)?;
                        self.chunk.push_constant_offset(null);
                    },
                    Some(ComponentParameter::Expression(expr)) => {
                        self.compile_expression(expr)?;
                    }
                    Some(ComponentParameter::Children(inner)) => {
                        self.compile_markup(inner)?;
                    }
                };

                self.chunk.push_op(OpCode::NewDictionary);
                let prop_map = self.scope.define_unnamed_local();
                for (name, expr) in &component.props {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(prop_map);
                    self.scope.define_unnamed_local();
                    let name = self.token_to_constant(name)?;
                    self.compile_expression(expr)?;
                    self.chunk.push_op(OpCode::SetField);
                    self.chunk.push_constant_offset(name);
                    self.scope.remove_top_n_locals(2);
                }
                let global = self.token_to_constant(&component.name)?;
                self.chunk.push_op(OpCode::GetGlobal);
                self.chunk.push_constant_offset(global);
                self.chunk.push_op(OpCode::Call);
                // 2 = arg count (content, props)
                self.chunk.push_u8(2);
                self.chunk.push_op(OpCode::NewNodeComponent);
                self.scope.remove_top_local();
            }
            Markup::Layout(_) => todo!(),
            Markup::Match(match_markup) => 'exit: {
                let null = self.add_to_constants(Value::Null)?;

                if match_markup.cases.is_empty() {
                    if let Some((name, markup)) = &match_markup.default_case {
                        if let Some(name) = name {
                            self.compile_expression(&match_markup.comparator)?;
                            let offset = self.scope.replace_local(&name.lexeme);
                            self.markup_declarations.insert(name.lexeme.to_owned(), offset);
                        }

                        self.compile_markup(&markup)?;

                        if let Some(name) = name {
                            self.chunk.push_op(OpCode::ReservePush);
                            self.chunk.push_op(OpCode::Pop);
                            self.chunk.push_op(OpCode::ReservePop);
                            self.scope.remove_top_local();
                            self.markup_declarations.remove(&name.lexeme);
                        }
                    } else {
                        self.chunk.push_op(OpCode::NewList);
                        self.chunk.push_op(OpCode::NewNodeBlock);
                        self.scope.define_unnamed_local();
                    }

                    break 'exit;
                }

                self.compile_expression(&match_markup.comparator)?;
                let comparator_expr = self.scope.get_top_local();

                if let Some((name, markup)) = &match_markup.default_case {
                    if let Some(name) = name {
                        self.compile_expression(&match_markup.comparator)?;
                        let offset = self.scope.replace_local(&name.lexeme);
                        self.markup_declarations.insert(name.lexeme.to_owned(), offset);
                    }

                    self.compile_markup(&markup)?;

                    if let Some(name) = name {
                        self.chunk.push_op(OpCode::ReservePush);
                        self.chunk.push_op(OpCode::Pop);
                        self.chunk.push_op(OpCode::ReservePop);
                        self.scope.remove_top_local();
                        self.markup_declarations.remove(&name.lexeme);
                    }
                } else {
                    self.chunk.push_op(OpCode::Constant);
                    self.chunk.push_constant_offset(null);
                    self.scope.define_unnamed_local();
                }

                for (expr, markup) in &match_markup.cases {
                    self.compile_markup(&markup)?;

                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(comparator_expr);
                    self.scope.define_unnamed_local();

                    self.compile_expression(expr)?;

                    self.chunk.push_op(OpCode::Eq);
                    self.chunk.push_op(OpCode::NewNodeIf);
                    self.scope.remove_top_n_locals(3);
                }

                self.chunk.push_op(OpCode::ReservePush);
                self.chunk.push_op(OpCode::Pop);
                self.chunk.push_op(OpCode::ReservePop);
                self.scope.remove_top_local();
            }
            Markup::If(markup) => {
                match &markup.else_markup {
                    Some(markup) => {
                        self.compile_markup(markup)?;
                    }
                    None => {
                        let null = self.add_to_constants(Value::Null)?;
                        self.chunk.push_op(OpCode::Constant);
                        self.chunk.push_constant_offset(null);
                        self.scope.define_unnamed_local();
                    }
                }

                self.compile_markup(&markup.if_markup)?;
                self.compile_expression(&markup.condition)?;
                self.chunk.push_op(OpCode::NewNodeIf);
                self.scope.remove_top_n_locals(2);
            }
            Markup::Iterative(markup) => {
                let mut chunk = Chunk::new("for_generator");
                let mut gen_compiler = Compiler::new(&mut chunk);

                for (name, offset) in &self.markup_declarations {
                    self.chunk.push_op(OpCode::GetLocal);
                    self.chunk.push_local_offset(*offset);
                    gen_compiler.scope.define_local(name);
                }

                self.chunk.push_op(OpCode::GetLocal);
                self.chunk.push_local_offset(0);

                gen_compiler.scope.define_local("self");

                if let Some(end) = &markup.end_range {
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

                    gen_compiler.chunk.push_op(OpCode::NewList);
                    let list = gen_compiler.scope.define_unnamed_local();

                    gen_compiler.compile_expression(&markup.iterable)?;
                    let start_local = gen_compiler.scope.replace_local(&markup.var_name.lexeme);
                    gen_compiler.compile_expression(end)?;
                    let end_local = gen_compiler.scope.get_top_local();

                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(end_local);
                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(start_local);
                    gen_compiler.chunk.push_op(OpCode::Lt);
                    let end_jump = gen_compiler.chunk.push_jump(OpCode::JumpIf);

                    let one = gen_compiler.add_to_constants(1)?;

                    let loop_addr = gen_compiler.chunk.addr();

                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(list);
                    gen_compiler.scope.define_unnamed_local();
                    gen_compiler.compile_markup(&markup.block)?;
                    gen_compiler.chunk.push_op(OpCode::PushList);
                    gen_compiler.scope.remove_top_n_locals(2);

                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(start_local);
                    gen_compiler.chunk.push_op(OpCode::Constant);
                    gen_compiler.chunk.push_constant_offset(one);
                    gen_compiler.chunk.push_op(OpCode::Add);
                    gen_compiler.chunk.push_op(OpCode::SetLocal);
                    gen_compiler.chunk.push_local_offset(start_local);
                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(start_local);
                    gen_compiler.chunk.push_op(OpCode::GetLocal);
                    gen_compiler.chunk.push_local_offset(end_local);
                    gen_compiler.chunk.push_op(OpCode::Lt);
                    gen_compiler.chunk.push_jump_im(OpCode::JumpIf, loop_addr);

                    gen_compiler.chunk.patch_jump(end_jump, gen_compiler.chunk.addr());

                    gen_compiler.chunk.push_op(OpCode::Pop);
                    gen_compiler.chunk.push_op(OpCode::Pop);

                    gen_compiler.chunk.push_op(OpCode::NewNodeBlock);
                    gen_compiler.chunk.push_op(OpCode::Return);
                } else {
                    todo!("Generic for loops not supported yet");
                }

                let func = Function {
                    chunk: Rc::new(chunk),
                    identifier: "for_generator".to_owned(),
                    arity: 0,
                    bound_value: None,
                };

                let constant = self.chunk.new_constant(func);
                self.chunk.push_op(OpCode::Constant);
                self.chunk.push_constant_offset(constant);
                self.chunk.push_op(OpCode::Bind);
                self.chunk.push_op(OpCode::Call);
                self.chunk.push_u8(self.markup_declarations.len() as u8);
                self.scope.define_unnamed_local();
            },
            Markup::Style(_) => todo!(),
        }

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
                match self.fetch_target(&accessor.expression)? {
                    VariableTarget::Local(local) => {
                        self.chunk.push_op(OpCode::GetLocal);
                        self.chunk.push_local_offset(local);
                        self.scope.define_unnamed_local();
                    }
                    VariableTarget::Global(global) => {
                        self.chunk.push_op(OpCode::GetGlobal);
                        self.chunk.push_constant_offset(global);
                        self.scope.define_unnamed_local();
                    }
                    VariableTarget::Object(obj) => {
                        self.chunk.push_op(OpCode::GetField);
                        self.chunk.push_constant_offset(obj);
                        self.scope.define_unnamed_local();
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
                Rc::new(
                    RefCell::new(
                        token.lexeme[1..token.lexeme.len() - 1].to_owned(),
                    )
                )
            )),
            TokenType::Identifier => Ok(Value::String(Rc::new(RefCell::new(token.lexeme.to_owned())))),

            _ => Err(CompileError::InvalidLiteral(token.lexeme.clone())),
        }
    }

    fn get_binary_operator(ty: TokenType) -> Result<OpCode, CompileError> {
        match ty {
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

    fn get_shorthand_operator(ty: TokenType) -> Result<OpCode, CompileError> {
        match ty {
            TokenType::IncrementAssign => Ok(OpCode::Add),
            TokenType::DecrementAssign => Ok(OpCode::Sub),
            TokenType::MulAssign => Ok(OpCode::Mul),
            TokenType::DivAssign => Ok(OpCode::Div),
            _ => Err(CompileError::InvalidShorthandOperator(ty)),
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
