use crate::statement::{ReturnStatement, Statement};
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
pub struct ArrayExpression {
    pub entries: Vec<Expression>,
}

#[derive(Debug)]
pub struct DictionaryExpression {
    pub entries: Vec<(Token, Expression)>,
}

#[derive(Debug)]
pub struct MatchExpression {
    pub comparator: Box<Expression>,
    pub cases: Vec<(Expression, Expression)>,
    pub default: Option<(Option<Token>, Box<Expression>)>
}

#[derive(Debug)]
pub struct IndexExpression {
    pub expression: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    FunctionCall(FunctionCallExpression),
    Accessor(AccessorExpression),
    Array(ArrayExpression),
    Dictionary(DictionaryExpression),
    Match(MatchExpression),
    Index(IndexExpression),
}

impl Into<Expression> for LiteralExpression {
    fn into(self) -> Expression {
        Expression::Literal(self)
    }
}

impl<'a> TryInto<&'a LiteralExpression> for &'a Expression {

    type Error = ();
    fn try_into(self) -> Result<&'a LiteralExpression, ()> {
        match self {
            Expression::Literal(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl Into<Expression> for BinaryExpression {
    fn into(self) -> Expression {
        Expression::Binary(self)
    }
}

impl<'a> TryInto<&'a BinaryExpression> for &'a Expression {

    type Error = ();
    fn try_into(self) -> Result<&'a BinaryExpression, ()> {
        match self {
            Expression::Binary(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl Into<Expression> for UnaryExpression {
    fn into(self) -> Expression {
        Expression::Unary(self)
    }
}

impl Into<Expression> for FunctionCallExpression {
    fn into(self) -> Expression {
        Expression::FunctionCall(self)
    }
}

impl Into<Expression> for AccessorExpression {
    fn into(self) -> Expression {
        Expression::Accessor(self)
    }
}

impl Into<Expression> for ArrayExpression {
    fn into(self) -> Expression {
        Expression::Array(self)
    }
}

impl Into<Expression> for DictionaryExpression {
    fn into(self) -> Expression {
        Expression::Dictionary(self)
    }
}

impl Into<Expression> for MatchExpression {
    fn into(self) -> Expression {
        Expression::Match(self)
    }
}

impl Into<Expression> for IndexExpression {
    fn into(self) -> Expression {
        Expression::Index(self)
    }
}

impl IndexExpression {
    pub fn new(expression: Expression, index: Expression) -> Self {
        Self {
            expression: Box::new(expression),
            index: Box::new(index),
        }
    }
}

impl LiteralExpression {
    pub fn new(token: Token) -> Self {
        Self {
            token,
        }
    }
}
impl BinaryExpression {
    pub fn new(lhs: Expression, op: Token, rhs: Expression) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl UnaryExpression {
    pub fn new(op: Token, rhs: Expression) -> Self {
        Self {
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl FunctionCallExpression {
    pub fn new(callee: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            callee: Box::new(callee),
            arguments,
        }
    }
}

impl AccessorExpression {
    pub fn new(expression: Expression, field: Token) -> Self {
        Self {
            expression: Box::new(expression),
            field,
        }
    }
}

impl ArrayExpression {
    pub fn new(entries: Vec<Expression>) -> Self {
        Self {
            entries,
        }
    }
}

impl DictionaryExpression {
    pub fn new(entries: Vec<(Token, Expression)>) -> Self {
        Self {
            entries,
        }
    }
}

impl MatchExpression {
    pub fn new(comparator: Expression, cases: Vec<(Expression, Expression)>, default_case: Option<(Option<Token>, Expression)>) -> Self {
        Self {
            comparator: Box::new(comparator),
            cases,
            default: default_case.map(|d| (d.0, Box::new(d.1))),
        }
    }
}