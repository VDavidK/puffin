use crate::token::{Token};
use crate::expression::{DictionaryExpression, Expression};
use crate::{VarType};
use crate::declaration::{Declaration, MethodDeclaration};

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug)]
pub struct AssignStatement {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub catch_block: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct BreakStatement;

#[derive(Debug)]
pub struct ContinueStatement;

#[derive(Debug)]
pub struct ThrowStatement {
    pub expression: Box<Expression>,
}

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
    pub catch_block: Option<Box<Statement>>,
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
    pub catch_block: Option<Box<Statement>>,
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
pub struct RaiseStatement;

#[derive(Debug)]
pub struct CatchStatement {
    pub cases: Vec<(Expression, Statement)>,
    pub default_case: Option<(Option<Token>, Box<Statement>)>,
}

#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    Assign(AssignStatement),
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
    Throw(ThrowStatement),
    Catch(CatchStatement),
    Raise(RaiseStatement),
}

impl From<BlockStatement> for Statement {
    fn from(s: BlockStatement) -> Self {
        Statement::Block(s)
    }
}

impl From<AssignStatement> for Statement {
    fn from(s: AssignStatement) -> Self {
        Statement::Assign(s)
    }
}

impl From<BreakStatement> for Statement {
    fn from(s: BreakStatement) -> Self {
        Statement::Break(s)
    }
}

impl From<ContinueStatement> for Statement {
    fn from(s: ContinueStatement) -> Self {
        Statement::Continue(s)
    }
}

impl From<ForStatement> for Statement {
    fn from(s: ForStatement) -> Self {
        Statement::For(s)
    }
}

impl From<IfStatement> for Statement {
    fn from(s: IfStatement) -> Self {
        Statement::If(s)
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(s: ExpressionStatement) -> Self {
        Statement::Expression(s)
    }
}

impl From<ReturnStatement> for Statement {
    fn from(s: ReturnStatement) -> Self {
        Statement::Return(s)
    }
}

impl From<MatchStatement> for Statement {
    fn from(s: MatchStatement) -> Self {
        Statement::Match(s)
    }
}

impl From<VariableDeclarationStatement> for Statement {
    fn from(s: VariableDeclarationStatement) -> Self {
        Statement::VariableDeclaration(s)
    }
}

impl From<IncrementStatement> for Statement {
    fn from(s: IncrementStatement) -> Self {
        Statement::Increment(s)
    }
}

impl From<DecrementStatement> for Statement {
    fn from(s: DecrementStatement) -> Self {
        Statement::Decrement(s)
    }
}

impl From<OpAssignStatement> for Statement {
    fn from(s: OpAssignStatement) -> Self {
        Statement::OpAssign(s)
    }
}

impl From<ThrowStatement> for Statement {
    fn from(s: ThrowStatement) -> Self {
        Statement::Throw(s)
    }
}

impl From<CatchStatement> for Statement {
    fn from(s: CatchStatement) -> Self {
        Statement::Catch(s)
    }
}

impl From<RaiseStatement> for Statement {
    fn from(s: RaiseStatement) -> Self {
        Statement::Raise(s)
    }
}

impl<'a> TryFrom<&'a Statement> for &'a ForStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::For(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Statement> for &'a ReturnStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, ()> {
        match value {
            Statement::Return(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Statement> for &'a BlockStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, ()> {
        match value {
            Statement::Block(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Statement> for &'a ExpressionStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, ()> {
        match value {
            Statement::Expression(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Statement> for &'a AssignStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, ()> {
        match value {
            Statement::Assign(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Statement> for &'a VariableDeclarationStatement {
    type Error = ();
    fn try_from(value: &'a Statement) -> Result<Self, ()> {
        match value {
            Statement::VariableDeclaration(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl CatchStatement {
    pub fn new(cases: Vec<(Expression, Statement)>) -> Self {
        Self {
            cases,
            default_case: None,
        }
    }
    pub fn with_default(mut self, case: (Option<Token>, Statement)) -> Self {
        self.default_case = Some((case.0, Box::new(case.1)));
        self
    }
}

impl ThrowStatement {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression: Box::new(expression),
        }
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
            rhs: Box::new(rhs),
            catch_block: None,
        }
    }

    pub fn with_catch(mut self, catch_block: Statement) -> Self {
        self.catch_block = Some(Box::new(catch_block));
        self
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
            catch_block: None,
        }
    }

    pub fn with_catch(mut self, catch_block: Statement) -> Self {
        self.catch_block = Some(Box::new(catch_block));
        self
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
            catch_block: None,
        }
    }
    pub fn with_catch(mut self, catch_block: Statement) -> Self {
        self.catch_block = Some(Box::new(catch_block));
        self
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