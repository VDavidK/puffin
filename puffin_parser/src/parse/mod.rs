use crate::lex::PuffinLexer;

use colored::Colorize;
use crate::lex;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error(transparent)]
    FileNotFoundError(#[from] std::io::Error),
    #[error(transparent)]
    LexerError(#[from] lex::LexerError),
}

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct PuffinParser<'a> {
    lexer: PuffinLexer<'a>,
}

impl<'a> PuffinParser<'a> {
    pub(crate) fn new(src: &'a str, src_name: &'a str) -> Self {
        PuffinParser {
            lexer: PuffinLexer::new(src, src_name),
        }
    }

    pub(crate) fn run(self) -> Result<(), ParserError> {
        let tokens = self.lexer.collect::<Result<Vec<_>, _>>()?;
        for token in tokens {
            println!("{}", format!("{}", token).green());
        }
        Ok(())
    }
}