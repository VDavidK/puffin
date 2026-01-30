use crate::lex::PuffinLexer;

use colored::Colorize;

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

    pub(crate) fn run(&mut self) {
        for token in &mut self.lexer {
            match token {
                Ok(token) => {
                    dbg!(token);
                },
                Err(err) => {
                    println!("{}", format!("{}", err).red());
                    break;
                },
            }
        }
    }
}