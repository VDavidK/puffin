use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use ratatui::crossterm::event::{KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui_image::{Image, Resize, StatefulImage};
use serde_derive::{Deserialize, Serialize};
use crate::event::{key_code_name, to_modifier_names, Event, EVENT_NAME_ONBUTTON, EVENT_NAME_ONFOCUSGAINED, EVENT_NAME_ONFOCUSLOST, EVENT_NAME_ONKEY, EVENT_NAME_ONPASTE, EVENT_NAME_ONRESIZE};
use crate::runtime::Runtime;
use crate::value::{InstanceType, IntType, Value};
use crate::{make_fields, RuntimeError};
use crate::value::ops::{ValueDef, ValueTruthy};

pub type NodeType = Rc<RefCell<Node>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Text(TextNode),
    Layout(LayoutNode),
    Frame(FrameNode),
    Component(ComponentNode),
    Conditional(ConditionalNode),
    Block(BlockNode),
    Image(ImageNode),
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
            Node::Image(img)
                => img.render(area, buf, state),
            Node::Layout(layout)
                => layout.render(area, buf, state),
            Node::Component(component)
                => component.render(area, buf, state),
            Node::Frame(item)
                => item.render(area, buf, state),
            Node::Block(block)
                => block.render(area, buf, state),
            Node::Conditional(_)
                => panic!("cannot directly render conditional nodes. must be wrapped in a layout"),
        }
    }
}

impl Node {
    pub fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        match self {
            Node::Layout(layout) => layout.dispatch(runtime, event)?,
            Node::Frame(frame) => frame.dispatch(runtime, event)?,
            Node::Component(component) => component.dispatch(runtime, event)?,
            Node::Conditional(component) => component.dispatch(runtime, event)?,
            Node::Block(component) => component.dispatch(runtime, event)?,
            _ => (),
        }
        Ok(())
    }

    fn expand(node: NodeType) -> Vec<NodeType> {
        if let Node::Conditional(cond) = &*node.borrow() {
            cond.expand()
        } else if let Node::Block(block) = &*node.borrow() {
            block.expand()
        } else {
            vec![node]
        }
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
    pub node: NodeType,
}

impl StatefulWidget for &FrameNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let frame = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let inner = frame.inner(area);
        let node = self.node.borrow();
        node.render(inner, buf, state);

        frame.render(area, buf);
    }
}


impl FrameNode {
    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        self.node.borrow().dispatch(runtime, event)?;
        Ok(())
    }
}

impl TryFrom<&Value> for Color {
    type Error = RuntimeError;

    fn try_from(value: &Value) -> Result<Color, RuntimeError> {
        match value.eval()? {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutNode {
    pub direction: LayoutDirection,
    pub node: NodeType,
    pub segments: Value,
}

impl StatefulWidget for &LayoutNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let nodes = Node::expand(self.node.to_owned());

        let len = nodes.len();
        let mut constraints = get_constraints(&self.segments, len).unwrap_or(vec![]);
        constraints.resize(len, Default::default());
        let direction = match self.direction {
            LayoutDirection::Vertical => Direction::Vertical,
            LayoutDirection::Horizontal => Direction::Horizontal,
        };

        let layout = Layout::new(direction, constraints)
            .split(area);

        for (node, area) in nodes.iter().zip(layout.iter()) {
            let node = node.borrow();
            node.render(*area, buf, state);
        }
    }
}

impl LayoutNode {
    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        let nodes = Node::expand(self.node.to_owned());
        for node in nodes {
            node.borrow().dispatch(runtime, event)?;
        }
        Ok(())
    }
}

pub(crate) fn get_constraints(constraints: &Value, child_count: usize) -> Result<Vec<Constraint>, RuntimeError> {
    let mut con = vec![];
    match constraints {
        Value::String(s) => {
            let c = to_constraint(
                &s.borrow()
                    .to_owned()
                    .split(":")
                    .collect::<Vec<_>>()
            )?;
            con.push(c);
            con.resize(child_count, Default::default());
            con.fill(c);
        },
        Value::List(l) => {
            for constraint in l.borrow().iter() {
                let c = constraint
                    .to_owned()
                    .take_string()?
                    .borrow()
                    .to_owned();
                let c = c
                    .split(":")
                    .collect::<Vec<_>>();
                let constraint = to_constraint(&c)?;
                con.push(constraint);
            }
            con.resize(child_count, Default::default());
        },
        t => return Err(RuntimeError::InvalidConstraint{ reason: format!("expected string or list, got {}", t.type_name())})
    }
    Ok(con)
}

fn to_constraint(values: &Vec<&str>) -> Result<Constraint, RuntimeError> {
    let name = values
        .get(0)
        .ok_or(RuntimeError::InvalidConstraint { reason: "missing constraint name".to_string() })?
        .to_owned();
    let value = usize::from_str(values.get(1)
        .ok_or(RuntimeError::InvalidConstraint { reason: "expected at least one value for constraint".to_string() })?.to_owned())?;
    let constraint = match name {
        "Length" => Constraint::Length(value as u16),
        "Fill" => Constraint::Fill(value as u16),
        "Min" => Constraint::Min(value as u16),
        "Max" => Constraint::Max(value as u16),
        "Percentage" => Constraint::Percentage(value as u16),
        "Ratio" => {
            let second_value = usize::from_str(values.get(2)
                .filter(|v| !v.is_empty())
                .ok_or(RuntimeError::InvalidConstraint { reason: "expected two values for Ratio".to_string() })?
                .to_owned())?;
            Constraint::Ratio(value as u32, second_value as u32)
        },
        name => return Err(RuntimeError::InvalidConstraintName{ name: name.to_owned() })
    };
    Ok(constraint)
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
        // Should not be able to fail, unless the user has defined an
        _ = self.instance.borrow_mut().set_field("bounds", make_fields! {
            x: area.x as IntType,
            y: area.y as IntType,
            columns: area.width as IntType,
            rows: area.height as IntType
        });

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
                ..
            }) => self.forward_event_fn(runtime, EVENT_NAME_ONKEY, || {
                make_fields! {
                    key: key_code_name(&code),
                    modifiers: to_modifier_names(modifiers)
                        .into_iter()
                        .map(Value::from)
                        .collect::<Vec<_>>()
                }
            })?,

            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(btn),
                column,
                row,
                modifiers,
            }) => self.forward_event_fn(runtime, EVENT_NAME_ONBUTTON, || {
                make_fields! {
                    button: match btn {
                        MouseButton::Left => "left",
                        MouseButton::Right => "right",
                        MouseButton::Middle => "middle"
                    },
                    column: *column as IntType,
                    row: *row as IntType,
                    modifiers: to_modifier_names(modifiers)
                        .into_iter()
                        .map(Value::from)
                        .collect::<Vec<_>>()
                }
            })?,

            Event::FocusGained => self.forward_event(runtime, EVENT_NAME_ONFOCUSGAINED, Value::Null)?,
            Event::FocusLost => self.forward_event(runtime, EVENT_NAME_ONFOCUSLOST, Value::Null)?,

            Event::Paste(buffer) => self.forward_event(runtime, EVENT_NAME_ONPASTE, make_fields! {
                content: buffer.to_owned(),
            })?,
            Event::Resize(rows, cols) => self.forward_event(runtime, EVENT_NAME_ONRESIZE, make_fields! {
                rows: *rows as IntType,
                columns: *cols as IntType,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalNode {
    pub condition: Value,
    pub if_case: NodeType,
    pub else_case: NodeType,
}

impl ConditionalNode {
    pub fn expand(&self) -> Vec<NodeType> {
        if self.truthy() {
            Node::expand(self.if_case.to_owned())
        } else {
            Node::expand(self.else_case.to_owned())
        }
    }

    pub fn truthy(&self) -> bool {
        self.condition.truthy().is_ok_and(|v| v)
    }

    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        let nodes = self.expand();
        for node in nodes {
            node.borrow().dispatch(runtime, event)?;
        }
        Ok(())
    }
}

impl From<ConditionalNode> for Value {
    fn from(value: ConditionalNode) -> Self {
        Node::Conditional(value).into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockNode {
    pub inner: Vec<NodeType>,
}

impl BlockNode {
    pub fn expand(&self) -> Vec<NodeType> {
        self.inner
            .to_owned()
            .into_iter()
            .map(Node::expand)
            .flatten()
            .collect()
    }

    fn dispatch(&self, runtime: &mut Runtime, event: &Event) -> Result<(), RuntimeError> {
        for node in &self.inner {
            node.borrow().dispatch(runtime, event)?;
        }
        Ok(())
    }
}

impl StatefulWidget for &BlockNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let nodes = self.inner
            .to_owned()
            .into_iter()
            .map(Node::expand)
            .flatten()
            .collect::<Vec<_>>();

        let len = nodes.len();
        let layout = Layout::new(Direction::Vertical, std::iter::repeat_n(Constraint::Fill(1), len))
            .split(area);

        for (node, area) in nodes.iter().zip(layout.iter()) {
            let node = node.borrow();
            node.render(*area, buf, state);
        }
    }
}

impl From<Vec<NodeType>> for BlockNode {
    fn from(value: Vec<NodeType>) -> Self {
        BlockNode { inner: value }
    }
}

impl From<BlockNode> for NodeType {
    fn from(value: BlockNode) -> Self {
        Rc::new(RefCell::new(Node::Block(value)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageNode {
    pub path: Value,
}

impl StatefulWidget for &ImageNode {
    type State = Runtime;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) where Self: Sized {
        let path = self.path
            .eval()
            .unwrap()
            .take_string()
            .unwrap();

        let img = state
            .get_protocol(&*path.borrow())
            .unwrap();

        StatefulImage::new()
            .resize(Resize::Scale(None))
            .render(area, buf, img);
    }
}

impl From<ImageNode> for Node {
    fn from(value: ImageNode) -> Self {
        Node::Image(value)
    }
}

