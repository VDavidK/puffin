use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use serde_derive::{Deserialize, Serialize};
use crate::runtime::Runtime;
use crate::value::{InstanceType, Value};
use crate::RuntimeError;
use crate::value::ops::{ValueDef, ValueTruthy};

pub type NodeType = Rc<RefCell<Node>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Text(TextNode),
    Layout(LayoutNode),
    Frame(FrameNode),
    Component(ComponentNode),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node")
    }
}

impl StatefulWidget for &Node {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized
    {
        match self {
            Node::Text(text)
            => text.render(area, buf),
            Node::Layout(layout)
            => layout.render(area, buf, state),
            Node::Component(component)
            => component.render(area, buf, state),
            Node::Frame(item)
            => item.render(area, buf, state),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub content: Value,
    pub text_color: Value,
    pub bg_color: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameNode {
    pub nodes: Vec<NodeType>,
}

impl StatefulWidget for &FrameNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let len = self.nodes.len();
        let frame = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let layout = Layout::new(Direction::Vertical, std::iter::repeat_n(Constraint::Fill(1), len))
            .split(frame.inner(area));
        frame.render(area, buf);

        for (node, area) in self.nodes.iter().zip(layout.iter()) {
            let node = node.borrow();
            node.render(*area, buf, state);
        }
    }
}

impl TryFrom<&Value> for Color {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Color, RuntimeError> {
        match value {
            Value::String(inner) => Ok(Color::from_str(inner.borrow().as_str())?),
            Value::Int(inner) => Ok(Color::from_u32(inner.to_owned() as u32)),
            _ => Err(RuntimeError::IncorrectType{ty: value.type_name().into(), expected: "hexadecimal string or integer".into() })
        }
    }
}

impl Widget for &TextNode {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let text_color = TryInto::try_into(&self.text_color).unwrap_or(Color::Reset).to_owned();
        let bg_color = TryInto::try_into(&self.bg_color).unwrap_or(Color::Reset).to_owned();
        let style = Style::new()
            .fg(text_color)
            .bg(bg_color);
        Paragraph::new(self.content.to_string())
            .set_style(style)
            .render(area, buf);
    }
}

impl From<TextNode> for Node {
    fn from(value: TextNode) -> Self {
        Node::Text(value)
    }
}

impl From<NodeType> for Value {
    fn from(value: NodeType) -> Self {
        Value::Node(value)
    }
}

impl From<Node> for Value {
    fn from(value: Node) -> Self {
        Rc::new(RefCell::new(value)).into()
    }
}

impl TryFrom<Value> for NodeType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Node(v) => Ok(v),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "node".to_owned() }),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Serialize, Deserialize)]
pub enum LayoutDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayoutNode {
    pub direction: LayoutDirection,
    pub nodes: Vec<NodeType>,
}

impl StatefulWidget for &LayoutNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let len = self.nodes.len();

        let direction = match self.direction {
            LayoutDirection::Vertical => Direction::Vertical,
            LayoutDirection::Horizontal => Direction::Horizontal,
        };

        let layout = Layout::new(direction, std::iter::repeat_n(Constraint::Fill(1), len))
            .split(area);

        for (node, area) in self.nodes.iter().zip(layout.iter()) {
            let node = node.borrow();
            node.render(*area, buf, state);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub instance: InstanceType,
}

impl From<ComponentNode> for NodeType {
    fn from(value: ComponentNode) -> Self {
        Rc::new(RefCell::new(Node::Component(value)))
    }
}

impl StatefulWidget for &ComponentNode {
    type State = Runtime;

    #[allow(clippy::unwrap_used)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // TODO: Fix error handling

        let instance = self.instance.borrow();
        let layout = instance
            .get_field("<layout>")
            .unwrap();

        let node = state.call(layout.to_owned(), &[instance.to_owned().into()])
            .unwrap()
            .take_node()
            .unwrap();

        node.borrow()
            .render(area, buf, state);
    }
}

impl ValueTruthy for NodeType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for NodeType {
    const TYPE_NAME: &'static str = "node";
}