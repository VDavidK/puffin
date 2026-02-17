use crate::token::{Token};
use crate::expression::{Expression};
use crate::{VarType};

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct AssignStatement {
    pub name: Token,
    pub expression: Box<Expression>
}

#[derive(Debug)]
pub struct VarStatement {
    pub name: Token,
    pub expression: Box<Expression>,
    pub var_type: VarType
}

#[derive(Debug)]
pub struct BreakStatement {}

#[derive(Debug)]
pub struct ContinueStatement {}

/*
#[derive(Debug)]
pub struct FunctionDeclaration
 { name: Box<Token>, parameters: Vec<Token> }
*/

#[derive(Debug)]
pub struct ForStatement {
    pub var_name: Token,
    pub iterable: Box<Expression>,
    pub end_range: Option<Box<Expression>>,
    pub block: Box<Statement>
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub if_block: Box<Statement>,
    pub else_stat: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    Assign(AssignStatement),
    Var(VarStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    /* FunctionDeclaration { name: Box<Token>, parameters: Vec<Token> }, */
    For(ForStatement),
    If(IfStatement),
    Expression(ExpressionStatement),
    Return(ReturnStatement),
}