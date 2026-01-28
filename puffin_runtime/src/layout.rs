use ratatui::prelude::*;

#[derive(Debug, Clone)]
pub enum Layout {
    Text(TextElement),
}

#[derive(Debug, Clone)]
pub struct TextElement {
    content: String,
}

impl Widget for &TextElement {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        Span::from(&self.content)
            .render(area, buf);
    }
}
