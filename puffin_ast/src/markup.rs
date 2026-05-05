use crate::expression::Expression;
use crate::token::Token;

#[derive(Debug)]
pub enum Markup {
    /// Direct rendering of a component
    Component(ComponentRender),
    Layout(LayoutRender),
    /// Rendering of a component via match-case
    Match(MatchConditionalRender),
    /// Rendering of a component via if/else
    If(IfConditionalRender),
    /// Rendering of an iterator of components
    Iterative(IterativeRender),
    /// Modifying the appearance of a component
    Style(StyleRender),
    /// A collection of multiple markup elements
    Block(MarkupBlock),
}

#[derive(Debug)]
pub enum ComponentParameter {
    Expression(Box<Expression>),
    Children(Box<Markup>),
}

#[derive(Debug)]
pub struct MarkupBlock {
    pub markup: Vec<Markup>,
}

#[derive(Debug)]
pub struct ComponentRender {
    pub name: Token,
    pub props: Vec<(Token, Box<Expression>)>,
    pub parameter: Option<ComponentParameter>,
}

#[derive(Debug)]
pub struct LayoutRender {
    pub name: Token,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub struct MatchConditionalRender {
    pub comparator: Box<Expression>,
    pub cases: Vec<(Expression, Vec<Markup>)>,
    pub default_case: Option<(Option<Token>, Vec<Markup>)>
}

#[derive(Debug)]
pub struct IfConditionalRender {
    pub condition: Box<Expression>,
    pub if_markup: Box<Markup>,
    pub else_markup: Option<Box<Markup>>,
}

#[derive(Debug)]
pub struct IterativeRender {
    pub var_name: Token,
    pub iterable: Box<Expression>,
    pub end_range: Option<Box<Expression>>,
    pub block: Option<Box<Markup>>
}

#[derive(Debug)]
pub struct StyleRender {
    pub values: Vec<(Token, Box<Expression>)>,
}

impl StyleRender {
    pub fn new(values: Vec<(Token, Expression)>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|(name, value)| (name.to_owned(), Box::new(value)))
                .collect(),
        }
    }
}

impl LayoutRender {
    pub fn new(name: Token, args: Vec<Expression>) -> Self {
        Self {
            name,
            args,
        }
    }
}

impl ComponentRender {
    pub fn new(name: Token, props: Vec<(Token, Expression)>) -> Self {
        Self {
            name,
            props: props.into_iter().map(|(t, e)| (t.to_owned(), Box::new(e))).collect::<Vec<(_, _)>>(),
            parameter: None,
        }
    }
    pub fn new_with_expression(name: Token, props: Vec<(Token, Expression)>, expression: Expression) -> Self {
        Self {
            parameter: Some(expression.into()),
            ..Self::new(name, props)
        }
    }
    pub fn new_with_children(name: Token, props: Vec<(Token, Expression)>, children: Markup) -> Self {
        Self {
            parameter: Some(children.into()),
            ..Self::new(name, props)
        }
    }
}

impl MatchConditionalRender {
    pub fn new(
        comparator: Expression,
        cases: Vec<(Expression, Vec<Markup>)>,
        default_case: Option<(Option<Token>, Vec<Markup>)>
    ) -> Self {
        Self {
            comparator: Box::new(comparator),
            cases,
            default_case,
        }
    }
}

impl IterativeRender {
    pub fn new(var_name: Token,
       iterable: Expression,
       end_range: Option<Expression>,
       block: Option<Markup>
    ) -> Self {
        Self {
            var_name,
            iterable: Box::new(iterable),
            end_range: end_range.map(Box::new),
            block: block.map(Box::new),
        }
    }
}

impl IfConditionalRender {
    pub fn new(condition: Expression, if_markup: Markup, else_markup: Option<Markup>) -> Self {
        Self {
            condition: Box::new(condition),
            if_markup: Box::new(if_markup),
            else_markup: else_markup.map(Box::new),
        }
    }
}

impl MarkupBlock {
    pub fn new(markup: Vec<Markup>) -> Self {
        Self {
            markup,
        }
    }
}

impl From<StyleRender> for Markup {
    fn from(m: StyleRender) -> Self {
        Markup::Style(m)
    }
}

impl From<ComponentRender> for Markup {
    fn from(m: ComponentRender) -> Self {
        Markup::Component(m)
    }
}
impl From<MatchConditionalRender> for Markup {
    fn from(m: MatchConditionalRender) -> Self {
        Markup::Match(m)
    }
}
impl From<IfConditionalRender> for Markup {
    fn from(m: IfConditionalRender) -> Self {
        Markup::If(m)
    }
}
impl From<IterativeRender> for Markup {
    fn from(m: IterativeRender) -> Self {
        Markup::Iterative(m)
    }
}

impl From<LayoutRender> for Markup {
    fn from(m: LayoutRender) -> Self {
        Markup::Layout(m)
    }
}

impl From<MarkupBlock> for Markup {
    fn from(m: MarkupBlock) -> Self {
        Markup::Block(m)
    }
}

impl From<Expression> for ComponentParameter {
    fn from(e: Expression) -> Self {
        ComponentParameter::Expression(Box::new(e))
    }
}

impl From<Markup> for ComponentParameter {
    fn from(m: Markup) -> Self {
        ComponentParameter::Children(Box::new(m))
    }
}