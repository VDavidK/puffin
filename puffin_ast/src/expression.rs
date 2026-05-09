use crate::statement::{ExpressionStatement, ReturnStatement, Statement};
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

impl From<LiteralExpression> for Expression {
    fn from(e: LiteralExpression) -> Self {
        Expression::Literal(e)
    }
}

impl<'a> TryFrom<&'a Expression> for &'a LiteralExpression {
    type Error = ();
    fn try_from(value: &'a Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Literal(l) => Ok(l),
            _ => Err(()),
        }
    }
}

impl TryFrom<Expression> for LiteralExpression {
    type Error = ();
    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Literal(l) => Ok(l),
            _ => Err(()),
        }
    }
}

impl From<BinaryExpression> for Expression {
    fn from(e: BinaryExpression) -> Self {
        Expression::Binary(e)
    }
}

impl<'a> TryFrom<&'a Expression> for &'a BinaryExpression {
    type Error = ();
    fn try_from(value: &'a Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Binary(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<UnaryExpression> for Expression {
    fn from(e: UnaryExpression) -> Self {
        Expression::Unary(e)
    }
}

impl<'a> TryFrom<&'a Expression> for &'a FunctionCallExpression {
    type Error = ();
    fn try_from(value: &'a Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::FunctionCall(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<FunctionCallExpression> for Expression {
    fn from(e: FunctionCallExpression) -> Self {
        Expression::FunctionCall(e)
    }
}

impl From<AccessorExpression> for Expression {
    fn from(e: AccessorExpression) -> Self {
        Expression::Accessor(e)
    }
}

impl From<ArrayExpression> for Expression {
    fn from(e: ArrayExpression) -> Self {
        Expression::Array(e)
    }
}

impl<'a> TryFrom<&'a Expression> for &'a DictionaryExpression {
    type Error = ();
    fn try_from(value: &'a Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Dictionary(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<DictionaryExpression> for Expression {
    fn from(e: DictionaryExpression) -> Self {
        Expression::Dictionary(e)
    }
}

impl From<MatchExpression> for Expression {
    fn from(e: MatchExpression) -> Self {
        Expression::Match(e)
    }
}

impl From<IndexExpression> for Expression {
    fn from(e: IndexExpression) -> Self {
        Expression::Index(e)
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