use std::fmt::Display;
use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Snippet {
    pub from: Option<String>,
    pub snippet: String,
    pub span: Span,
    pub start_line: usize,
}

impl Display for Snippet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format = if let Some(from) = &self.from {
            format!(" in '{}:{}:{}'\n", from, self.span.start.ln(), self.span.start.cn())
        } else {
            format!(" at '{}:{}'\n", self.span.start.ln(), self.span.start.cn())
        };

        let lines = self.snippet
            .lines()
            .collect::<Vec<_>>();

        let line_nums = (self.start_line..self.start_line + lines.len())
            .collect::<Vec<_>>();

        let largest_num_digits = line_nums
            .iter()
            .max()
            .unwrap_or(&0)
            .checked_ilog10()
            .unwrap_or(0) as usize + 1;

        let multiline = self.span.line_span() > 1;

        for (line, ln) in lines.iter().zip(line_nums.iter()) {
            format.push_str(&format!("{ln:>largest_num_digits$} | {line}\n"));

            if *ln < self.span.start.ln() || *ln > self.span.end.ln() {
                continue;
            }

            format.push_str(&format!("{:>largest_num_digits$} | ", ""));

            if *ln == self.span.start.ln() {
                // Padded start
                let space = " ".repeat(self.span.start.cn() - 1);
                let arrow_end = if multiline { line.len() + 1 } else { self.span.end.cn() };
                let arrows = "^".repeat(arrow_end - self.span.start.cn() + 1);
                format.push_str(&format!("{space}{arrows}"));
            } else if *ln == self.span.end.ln() {
                // "Padded" end
                let arrow_start = if multiline { 1 } else { self.span.end.cn() };
                let arrows = "^".repeat(self.span.end.cn() - arrow_start + 1);
                format.push_str(&arrows);
            } else if multiline {
                // Full arrows
                let arrows = "^".repeat(line.len());
                format.push_str(&arrows);
            }

            if *ln == self.span.end.ln() {
                format.push_str(" here");
            }

            format.push('\n');
        }
        f.write_str(&format)
    }
}

pub trait IntoSnippet {
    fn into_snippet(self, src: impl AsRef<str>, src_name: Option<impl AsRef<str>>, ctx_ln_count: usize) -> Snippet;
}