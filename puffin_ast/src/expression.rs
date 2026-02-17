use crate::token::{Token};

#[derive(Debug)]
pub struct LiteralExpression {
    pub token: Token,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub op: Token,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub op: Token,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionCallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct AccessorExpression {
    pub expression: Box<Expression>,
    pub field: Token
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    FunctionCall(FunctionCallExpression),
    Accessor(AccessorExpression),
}

impl LiteralExpression {
    pub fn new(token: Token) -> Self {
        Self {
            token,
        }
    }
}
impl BinaryExpression {
    pub fn new(lhs: Box<Expression>, op: Token, rhs: Box<Expression>) -> Self {
        Self {
            lhs,
            op,
            rhs,
        }
    }
}

impl UnaryExpression {
    pub fn new(op: Token, rhs: Box<Expression>) -> Self {
        Self {
            op,
            rhs,
        }
    }
}

impl FunctionCallExpression {
    pub fn new(callee: Box<Expression>, arguments: Vec<Expression>) -> Self {
        Self {
            callee,
            arguments,
        }
    }
}

impl AccessorExpression {
    pub fn new(expression: Box<Expression>, field: Token) -> Self {
        Self {
            expression,
            field,
        }
    }
}