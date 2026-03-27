use crate::token::{Token};
use crate::expression::{Expression};
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

impl From<RaiseStatement> for Statement {
    fn from(m: RaiseStatement) -> Self {
        Statement::Raise(m)
    }
}

impl From<CatchStatement> for Statement {
    fn from(m: CatchStatement) -> Self {
        Statement::Catch(m)
    }
}
impl From<ThrowStatement> for Statement {
    fn from(m: ThrowStatement) -> Self {
        Statement::Throw(m)
    }
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
impl<'a> TryInto<&'a ReturnStatement> for &'a Statement {
    type Error = ();
    fn try_into(self) -> Result<&'a ReturnStatement, ()> {
        match self {
            Statement::Return(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a BlockStatement> for &'a Statement {

    type Error = ();
    fn try_into(self) -> Result<&'a BlockStatement, ()> {
        match self {
            Statement::Block(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a ExpressionStatement> for &'a Statement {

    type Error = ();
    fn try_into(self) -> Result<&'a ExpressionStatement, ()> {
        match self {
            Statement::Expression(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a AssignStatement> for &'a Statement {

    type Error = ();
    fn try_into(self) -> Result<&'a AssignStatement, ()> {
        match self {
            Statement::Assign(c) => Ok(c),
            _ => Err(()),
        }
    }
}

impl<'a> TryInto<&'a VariableDeclarationStatement> for &'a Statement {

    type Error = ();
    fn try_into(self) -> Result<&'a VariableDeclarationStatement, ()> {
        match self {
            Statement::VariableDeclaration(c) => Ok(c),
            _ => Err(()),
        }
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