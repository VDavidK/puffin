use crate::token::{Token};
use crate::expression::{Expression};
use crate::{VarType};

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct AssignStatement {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>
}

#[derive(Debug)]
pub struct VarStatement {
    pub name: Token,
    pub expression: Box<Expression>,
    pub var_type: VarType
}

#[derive(Debug)]
pub struct BreakStatement;

#[derive(Debug)]
pub struct ContinueStatement;

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
pub struct MatchStatement {
    pub comparator: Box<Expression>,
    pub cases: Vec<(Expression, Statement)>,
    pub default_case: Option<(Option<Token>, Box<Statement>)>
}

#[derive(Debug)]
pub struct VariableDeclarationStatement {
    pub name: Token,
    pub value: Box<Expression>,
    pub var_type: VarType,
}

#[derive(Debug)]
pub struct IncrementStatement {
    pub target: Box<Expression>,
}

#[derive(Debug)]
pub struct DecrementStatement {
    pub target: Box<Expression>,
}

#[derive(Debug)]
pub struct OpAssignStatement {
    pub lhs: Box<Expression>,
    pub op: Token,
    pub rhs: Box<Expression>,
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
    Match(MatchStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Increment(IncrementStatement),
    Decrement(DecrementStatement),
    OpAssign(OpAssignStatement),
}

impl From<BlockStatement> for Statement {
    fn from(m: BlockStatement) -> Self {
        Statement::Block(m)
    }
}
impl From<AssignStatement> for Statement {
    fn from(m: AssignStatement) -> Self {
        Statement::Assign(m)
    }
}
impl From<VarStatement> for Statement {
    fn from(m: VarStatement) -> Self {
        Statement::Var(m)
    }
}
impl From<BreakStatement> for Statement {
    fn from(m: BreakStatement) -> Self {
        Statement::Break(m)
    }
}
impl From<ContinueStatement> for Statement {
    fn from(m: ContinueStatement) -> Self {
        Statement::Continue(m)
    }
}
impl From<ForStatement> for Statement {
    fn from(m: ForStatement) -> Self {
        Statement::For(m)
    }
}
impl From<IfStatement> for Statement {
    fn from(m: IfStatement) -> Self {
        Statement::If(m)
    }
}
impl From<ExpressionStatement> for Statement {
    fn from(m: ExpressionStatement) -> Self {
        Statement::Expression(m)
    }
}
impl From<ReturnStatement> for Statement {
    fn from(m: ReturnStatement) -> Self {
        Statement::Return(m)
    }
}

impl From<MatchStatement> for Statement {
    fn from(m: MatchStatement) -> Self {
        Statement::Match(m)
    }
}

impl From<VariableDeclarationStatement> for Statement {
    fn from(m: VariableDeclarationStatement) -> Self {
        Statement::VariableDeclaration(m)
    }
}

impl From<IncrementStatement> for Statement {
    fn from(m: IncrementStatement) -> Self {
        Statement::Increment(m)
    }
}

impl From<DecrementStatement> for Statement {
    fn from(m: DecrementStatement) -> Self {
        Statement::Decrement(m)
    }
}

impl From<OpAssignStatement> for Statement {
    fn from(m: OpAssignStatement) -> Self {
        Statement::OpAssign(m)
    }
}

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
        }
    }
}
impl AssignStatement {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}
impl VarStatement {
    pub fn new(name: Token, expression: Expression, var_type: VarType) -> Self {
        Self {
            name,
            expression: Box::new(expression),
            var_type,
        }
    }
}
impl ForStatement {
    pub fn new(var_name: Token, iterable: Expression, end_range: Option<Expression>, block: Statement) -> Self {
        Self {
            var_name,
            iterable: Box::new(iterable),
            block: Box::new(block),
            end_range: end_range.map(Box::new),
        }
    }
}
impl IfStatement {
    pub fn new(condition: Expression, if_block: Statement, else_stat: Option<Statement>) -> Self {
        Self {
            condition: Box::new(condition),
            if_block: Box::new(if_block),
            else_stat: else_stat.map(Box::new),
        }
    }
}
impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}
impl ReturnStatement {
    pub fn new(expression: Option<Expression>) -> Self {
        Self {
            expression: expression.map(Box::new),
        }
    }
}

impl MatchStatement {
    pub fn new(comparator: Expression, cases: Vec<(Expression, Statement)>, default_case: Option<(Option<Token>, Statement)>) -> Self {
        Self {
            comparator: Box::new(comparator),
            cases,
            default_case: default_case.map(|c| (c.0, Box::new(c.1))),
        }
    }
}

impl VariableDeclarationStatement {
    pub fn new(name: Token, value: Expression, var_type: VarType) -> Self {
        Self {
            name,
            value: Box::new(value),
            var_type,
        }
    }
}

impl IncrementStatement {
    pub fn new(target: Expression) -> Self {
        Self {
            target: Box::new(target),
        }
    }
}

impl DecrementStatement {
    pub fn new(target: Expression) -> Self {
        Self {
            target: Box::new(target),
        }
    }
}


impl OpAssignStatement {
    pub fn new(lhs: Expression, op: Token, rhs: Expression) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}