use ratatui::prelude::*;
use crate::{RuntimeError, Value};
use crate::value::ElementType;

#[derive(Debug, Clone)]
pub enum Element {
    Text(TextElement),
    Layout(LayoutElement),
    Component(ComponentElement),
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl TryFrom<Value> for ElementType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Element(component) => Ok(component),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "component".to_owned() }),
        }
    }
}

impl Widget for &Element {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        match self {
            Element::Text(text)
                => text.render(area, buf),
            Element::Layout(layout)
                => layout.render(area, buf),
            Element::Component(component)
                => component.render(area, buf),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextElement {
    pub content: String,
}

impl Widget for &TextElement {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        Span::from(&self.content)
            .render(area, buf);
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum LayoutDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutElement {
    pub direction: LayoutDirection,
    pub components: Vec<ElementType>,
}

impl Widget for &LayoutElement {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let len = self.components.len();

        // TODO: Implement
    }
}

#[derive(Debug, Clone)]
pub struct ComponentElement {
    pub root: Box<Element>,
}

impl Widget for &ComponentElement {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        self.root.render(area, buf);
    }
}
