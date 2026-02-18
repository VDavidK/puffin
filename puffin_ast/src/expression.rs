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
pub enum Expression {
    Literal(LiteralExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    FunctionCall(FunctionCallExpression),
    Accessor(AccessorExpression),
    Array(ArrayExpression),
    Dictionary(DictionaryExpression),
    Match(MatchExpression),
}

impl From<LiteralExpression> for Expression {
    fn from(m: LiteralExpression) -> Self {
        Expression::Literal(m)
    }
}

impl From<BinaryExpression> for Expression {
    fn from(m: BinaryExpression) -> Self {
        Expression::Binary(m)
    }
}

impl From<UnaryExpression> for Expression {
    fn from(m: UnaryExpression) -> Self {
        Expression::Unary(m)
    }
}

impl From<FunctionCallExpression> for Expression {
    fn from(m: FunctionCallExpression) -> Self {
        Expression::FunctionCall(m)
    }
}

impl From<AccessorExpression> for Expression {
    fn from(m: AccessorExpression) -> Self {
        Expression::Accessor(m)
    }
}

impl From<ArrayExpression> for Expression {
    fn from(m: ArrayExpression) -> Self {
        Expression::Array(m)
    }
}

impl From<DictionaryExpression> for Expression {
    fn from(m: DictionaryExpression) -> Self {
        Expression::Dictionary(m)
    }
}

impl From<MatchExpression> for Expression {
    fn from(m: MatchExpression) -> Self {
        Expression::Match(m)
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