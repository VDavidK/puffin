use crate::snippet::{IntoSnippet, Snippet};
use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    ln: usize,
    cn: usize,
    idx: usize,
}

impl Position {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ln(&self) -> usize {
        self.ln
    }

    pub fn cn(&self) -> usize {
        self.cn
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn move_forward_by(&mut self, source: impl AsRef<str>, count: usize) -> bool {
        for _ in 0..count {
            if !self.move_forward(source.as_ref()) {
                return false;
            }
        }

        true
    }

    pub fn move_forward(&mut self, source: impl AsRef<str>) -> bool {
        let char = source
            .as_ref()
            .chars()
            .nth(self.idx);

        if let Some(char) = char {
            self.idx += 1;
            self.cn += 1;

            if char == '\n' {
                self.ln += 1;
                self.cn = 1;
            }

            true
        } else {
            false
        }
    }
}

impl IntoSnippet for Position {
    fn into_snippet(self, src: impl AsRef<str>, src_name: Option<impl AsRef<str>>, ctx_ln_count: usize) -> Snippet {
        let snippet_start_line = i32::max(1, self.ln as i32 - ctx_ln_count as i32) as usize;

        let snippet: String = src
            .as_ref()
            .lines()
            .skip(snippet_start_line - 1)
            .take(ctx_ln_count * 2 + 1)
            .fold(String::new(), |a, b| format!("{a}{b}\n"));

        Snippet {
            from: src_name.map(|s| s.as_ref().to_owned()),
            span: Span::from_positions(self, self),
            snippet,
            start_line: snippet_start_line,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("line {}, column {}", self.ln, self.cn))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.ln == other.ln
            && self.cn == other.cn
            && self.idx == other.idx
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            ln: 1,
            cn: 1,
            idx: 0,
        }
    }
}
