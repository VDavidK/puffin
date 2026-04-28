use std::fmt::{Display, Formatter};
use ratatui::prelude::*;
use serde_derive::{Deserialize, Serialize};
use crate::value::NodeType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Text(TextNode),
    Layout(LayoutNode),
    Component(ComponentNode),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node")
    }
}

impl Widget for &Node {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        match self {
            Node::Text(text)
            => text.render(area, buf),
            Node::Layout(layout)
            => layout.render(area, buf),
            Node::Component(component)
            => component.render(area, buf),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextNode {
    pub content: String,
}

impl Widget for &TextNode {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        Span::from(&self.content)
            .render(area, buf);
    }
}

impl From<TextNode> for Node {
    fn from(value: TextNode) -> Self {
        Node::Text(value)
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

impl Widget for &LayoutNode {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let len = self.nodes.len();

        let direction = match self.direction {
            LayoutDirection::Vertical => Direction::Vertical,
            LayoutDirection::Horizontal => Direction::Horizontal,
        };

        let layout = Layout::new(direction, std::iter::repeat_n(Constraint::Fill(1), len))
            .split(area);

        for (node, area) in self.nodes.iter().zip(layout.iter()) {
            node.borrow().render(*area, buf);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    pub root: Box<Node>,
}

impl Widget for &ComponentNode {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        self.root.render(area, buf);
    }
}
