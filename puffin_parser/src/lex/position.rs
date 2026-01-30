use super::snippet::Snippet;

#[derive(Debug, Clone)]
pub struct Position {
    ln: usize,
    cn: usize,
    idx: usize,
    snippet: Option<Snippet>,
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

    pub fn with_snippet(mut self, source: impl AsRef<str>, from: impl Into<String>, extra_lines: usize) -> Self {
        self.attach_snippet(source, from, extra_lines);
        self
    }

    pub fn attach_snippet(&mut self, source: impl AsRef<str>, from: impl Into<String>, extra_lines: usize) {
        let snippet_start_line = i32::max(1, self.ln as i32 - extra_lines as i32) as usize;

        let snippet: String = source
            .as_ref()
            .lines()
            .skip(snippet_start_line - 1)
            .take(extra_lines * 2 + 1)
            .fold(String::new(), |a, b| format!("{a}{b}\n"));

        self.snippet = Some(Snippet {
            from: from.into(),
            snippet,
            snippet_start_line,
        });
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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format = format!("line {}, column {}", self.ln, self.cn);

        if let Some(snippet) = &self.snippet {
            format.push_str(&format!(" in '{}:{}:{}'\n", snippet.from, self.ln, self.cn));

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

            for (line, ln) in lines.iter().zip(line_nums.iter()) {
                if *ln == self.ln {
                    let space = " ".repeat(self.cn - 1);
                    format.push_str(&format!("{ln:>largest_num_digits$} | {line}\n"));
                    format.push_str(&format!("{:>largest_num_digits$} | {space}^-- here\n", ""));
                } else {
                    format.push_str(&format!("{ln:>largest_num_digits$} | {line}\n"));
                }
            }
        }

        f.write_str(format.as_str())
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
            snippet: None,
        }
    }
}
