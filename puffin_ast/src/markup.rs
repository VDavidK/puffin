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
}

#[derive(Debug)]
pub enum MarkupProp {
    DirectBindings(DirectBindings),
    Lambda(LambdaFunctionBinding),
}

#[derive(Debug)]
pub struct LambdaFunctionBinding {
    pub parameters: Vec<Token>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct DirectBindings {
    pub names: Vec<Token>,
}

#[derive(Debug)]
pub struct MarkupBinding {
    pub name: Token,
    pub binding: MarkupProp,
}

#[derive(Debug)]
pub struct ComponentRender {
    pub name: Token,
    pub bindings: Vec<MarkupBinding>,
    pub string_literal: Option<Token>,
    pub children: Vec<Markup>,
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
    pub if_markup: Vec<Markup>,
    pub elseif_markup: Option<Box<Markup>>,
    pub else_markup: Vec<Markup>,
}

#[derive(Debug)]
pub struct IterativeRender {
    pub var_name: Token,
    pub iterable: Box<Expression>,
    pub end_range: Option<Box<Expression>>,
    pub block: Vec<Markup>
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

impl LambdaFunctionBinding {
    pub fn new(parameters: Vec<Token>, expressions: Vec<Expression>) -> Self {
        Self {
            parameters,
            expressions,
        }
    }
}

impl DirectBindings {
    pub fn new(names: Vec<Token>) -> Self {
        Self {
            names,
        }
    }
}

impl MarkupBinding {
    pub fn new(name: Token, binding: MarkupProp) -> Self {
        Self {
            name,
            binding,
        }
    }
}

impl ComponentRender {
    pub fn new(name: Token, bindings: Vec<MarkupBinding>, string_literal: Option<Token>, children: Vec<Markup>,) -> Self {
        Self {
            name,
            bindings,
            string_literal,
            children,
        }
    }

    pub fn with_string_literal(&mut self, string_literal: Token) -> &Self {
        self.string_literal = Some(string_literal);
        self
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
       block: Vec<Markup>
    ) -> Self {
        Self {
            var_name,
            iterable: Box::new(iterable),
            end_range: end_range.map(Box::new),
            block,
        }
    }
}

impl IfConditionalRender {
    pub fn new(condition: Expression, if_markup: Vec<Markup>, elseif_markup: Option<Markup>, else_markup: Vec<Markup>) -> Self {
        Self {
            condition: Box::new(condition),
            if_markup,
            elseif_markup: elseif_markup.map(Box::new),
            else_markup,
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
impl From<DirectBindings> for MarkupProp {
    fn from(m: DirectBindings) -> Self {
        MarkupProp::DirectBindings(m)
    }
}
impl From<LambdaFunctionBinding> for MarkupProp {
    fn from(m: LambdaFunctionBinding) -> Self {
        MarkupProp::Lambda(m)
    }
}

impl From<LayoutRender> for Markup {
    fn from(m: LayoutRender) -> Self {
        Markup::Layout(m)
    }
}