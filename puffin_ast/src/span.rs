use crate::snippet::{Snippet, IntoSnippet};
use crate::position::Position;

#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl IntoSnippet for Span {
    fn into_snippet(self, src: impl AsRef<str>, src_name: Option<impl AsRef<str>>, ctx_ln_count: usize) -> Snippet {
        let snippet_start_line = i32::max(1, self.start.ln() as i32 - ctx_ln_count as i32) as usize;
        let line_span = self.end.ln() - self.start.ln() + 1;

        let snippet: String = src
            .as_ref()
            .lines()
            .skip(snippet_start_line - 1)
            .take(ctx_ln_count * 2 + line_span)
            .fold(String::new(), |a, b| format!("{a}{b}\n"));

        Snippet {
            from: src_name.map(|s| s.as_ref().to_owned()),
            snippet,
            span: self,
            start_line: snippet_start_line,
        }
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.end == other.end
    }
}

impl Span {
    pub fn from_positions(start: Position, end: Position) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn line_span(&self) -> usize {
        self.end.ln() - self.start.ln() + 1
    }

    pub fn with_snippet(mut self, source: impl AsRef<str>, from: impl Into<String>, extra_lines: usize) -> Self {
        self.attach_snippet(source, from, extra_lines);
        self
    }

    pub fn attach_snippet(&mut self, source: impl AsRef<str>, from: impl Into<String>, extra_lines: usize) {
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format = format!("line {}, column {}", self.start.ln(), self.end.cn());

        if self.line_span() > 1 {
            format.push_str(&format!(" to line {}, column {}", self.end.ln(), self.end.cn()));
        }

        f.write_str(format.as_str())
    }
}
