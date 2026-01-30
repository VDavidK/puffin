use crate::parse::PuffinParser;

mod lex;
mod parse;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error(transparent)]
    FileNotFoundError(#[from] std::io::Error),
    #[error(transparent)]
    LexerError(#[from] lex::LexerError),
}

/// Runs the parser (and lexer) on the given string.
pub fn run_parser_str(src: impl AsRef<str>) -> Result<(), ParserError> {
    run_parser(src, "<anonymous>")
}

/// Functionally identical to run_parser_str, but opens a file instead.
pub fn run_parser_file(file_path: &str) -> Result<(), ParserError> {
    let src = std::fs::read_to_string(file_path)?;
    run_parser(src, file_path)
}

/// The actual function responsible for running the lexer and parser.
fn run_parser(src: impl AsRef<str>, file_path: impl AsRef<str>) -> Result<(), ParserError> {
    let mut parser = PuffinParser::new(src.as_ref(), file_path.as_ref());

    parser.run();
    Ok(())
}