use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use ratatui::crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use serde_derive::{Deserialize, Serialize};
use crate::event::{key_code_name, Event, EVENT_NAME_ONKEY};
use crate::runtime::Runtime;
use crate::value::{InstanceType, Module, Value};
use crate::{make_dict_value, make_fields, RuntimeError};
use crate::value::ops::{ValueDef, ValueTruthy};
use crate::value::Value::Dictionary;

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

impl Node {
    pub fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        match self {
            Node::Layout(layout) => layout.dispatch(runtime, event)?,
            Node::Frame(frame) => frame.dispatch(runtime, event)?,
            Node::Component(component) => component.dispatch(runtime, event)?,
            _ => (),
        }
        Ok(())
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub content: Value,
    pub text_color: Value,
    pub bg_color: Value,
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


impl FrameNode {
    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        for node in &self.nodes {
            node.borrow().dispatch(runtime, event)?;
        }
        Ok(())
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

impl LayoutNode {
    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        for node in &self.nodes {
            node.borrow().dispatch(runtime, event)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub instance: InstanceType,
    pub root: NodeType,
}

impl From<ComponentNode> for NodeType {
    fn from(value: ComponentNode) -> Self {
        Rc::new(RefCell::new(Node::Component(value)))
    }
}

impl TryFrom<InstanceType> for ComponentNode {
    type Error = RuntimeError;

    fn try_from(value: InstanceType) -> Result<Self, Self::Error> {
        let node = ComponentNode {
            instance: value.to_owned(),
            root: value
                .borrow()
                .get_field("<layout>")
                .cloned()
                .unwrap() // TODO: Better
                .take_node()?,
        };

        Ok(node)
    }
}

impl StatefulWidget for &ComponentNode {
    type State = Runtime;

    #[allow(clippy::unwrap_used)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.root.borrow()
            .render(area, buf, state);
    }
}

impl ComponentNode {
    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        match event {
            // Key press event
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                state
            }) => self.forward_event_fn(runtime, EVENT_NAME_ONKEY, || {
                make_fields! {
                    key: key_code_name(&code),
                }
            })?,

            _ => (),
        }

        // Propagate to child nodes
        self.root.borrow().dispatch(runtime, event)?;

        Ok(())
    }

    fn forward_event(&self, runtime: &mut Runtime, name: impl AsRef<str>, event_value: impl Into<Value>) -> Result<(), RuntimeError> {
        let handler = self.instance.borrow().get_handler(name);
        if let Some(handler) = handler {
            runtime.call(handler, &[event_value.into()])?;
        }
        Ok(())
    }

    fn forward_event_fn<F: FnOnce() -> R, R: Into<Value>>(&self, runtime: &mut Runtime, name: impl AsRef<str>, constructor: F) -> Result<(), RuntimeError> {
        self.forward_event(runtime, name, constructor())
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