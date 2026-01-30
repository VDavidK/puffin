#[derive(Debug, Clone)]
pub struct Snippet {
    pub from: String,
    pub snippet: String,
    pub snippet_start_line: usize,
}