use crate::snippet::Snippet;
use crate::position::Position;

#[derive(Debug, Clone, Default)]
pub struct Span {
    start: Position,
    end: Position,
    snippet: Option<Snippet>,
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
            snippet: None,
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
        let snippet_start_line = i32::max(1, self.start.ln() as i32 - extra_lines as i32) as usize;
        let line_span = self.end.ln() - self.start.ln() + 1;

        let snippet: String = source
            .as_ref()
            .lines()
            .skip(snippet_start_line - 1)
            .take(extra_lines * 2 + line_span)
            .fold(String::new(), |a, b| format!("{a}{b}\n"));

        self.snippet = Some(Snippet {
            from: from.into(),
            snippet,
            snippet_start_line,
        });
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format = format!("line {}, column {}", self.start.ln(), self.end.cn());

        if self.line_span() > 1 {
            format.push_str(&format!(" to line {}, column {}", self.end.ln(), self.end.cn()));
        }

        if let Some(snippet) = &self.snippet {
            format.push_str(&format!(" in '{}:{}:{}'\n", snippet.from, self.start.ln(), self.start.cn()));

            let lines = snippet.snippet
                .lines()
                .collect::<Vec<_>>();

            let line_nums = (snippet.snippet_start_line..snippet.snippet_start_line + lines.len())
                .collect::<Vec<_>>();

            let largest_num_digits = line_nums
                .iter()
                .max()
                .unwrap_or(&0)
                .checked_ilog10()
                .unwrap_or(0) as usize + 1;

            let multiline = self.line_span() > 1;

            for (line, ln) in lines.iter().zip(line_nums.iter()) {
                format.push_str(&format!("{ln:>largest_num_digits$} | {line}\n"));

                if *ln < self.start.ln() || *ln > self.end.ln() {
                    continue;
                }

                format.push_str(&format!("{:>largest_num_digits$} | ", ""));

                if *ln == self.start.ln() {
                    // Padded start
                    let space = " ".repeat(self.start.cn() - 1);
                    let arrow_end = if multiline { line.len() + 1 } else { self.end.cn() };
                    let arrows = "^".repeat(arrow_end - self.start.cn());
                    format.push_str(&format!("{space}{arrows}"));
                } else if *ln == self.end.ln() {
                    // "Padded" end
                    let arrow_start = if multiline { 1 } else { self.end.cn() };
                    let arrows = "^".repeat(self.end.cn() - arrow_start);
                    format.push_str(&format!("{arrows}"));
                } else if multiline {
                    // Full arrows
                    let arrows = "^".repeat(line.len());
                    format.push_str(&format!("{arrows}"));
                }

                if *ln == self.end.ln() {
                    format.push_str(" here");
                }

                format.push_str("\n");
            }
        }

        f.write_str(format.as_str())
    }
}
