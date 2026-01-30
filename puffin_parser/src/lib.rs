mod lex;
mod parse;

fn run_lexer() -> Result<(), ()> {
    let lexer = lex::PuffinLexer::new();
    Ok(())
}
